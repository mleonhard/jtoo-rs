use crate::escape_and_elide;
use core::fmt::Debug;

#[derive(Eq, PartialEq)]
pub enum DecodeError {
    DataNotConsumed(Vec<u8>),
    ExpectedBool(Vec<u8>),
    ExpectedListEnd(Vec<u8>),
    ExpectedListSeparator(Vec<u8>),
    ExpectedNoMoreData(Vec<u8>),
    ExpectedString(Vec<u8>),
    IncompleteEscape(Vec<u8>),
    InvalidEscape(Vec<u8>),
    ListEndNotConsumed(Vec<u8>),
    NotInList(Vec<u8>),
    NotUtf8(Vec<u8>),
    UnclosedString(Vec<u8>),
}
impl Debug for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        let (prefix, b) = match self {
            DecodeError::DataNotConsumed(b) => ("DecodeError: program error: data not consumed", b),
            DecodeError::ExpectedBool(b) => ("DecodeError: expected bool, got", b),
            DecodeError::ExpectedListEnd(b) => ("DecodeError: expected list end, got", b),
            DecodeError::ExpectedListSeparator(b) => ("DecodeError: expected comma, got", b),
            DecodeError::ExpectedNoMoreData(b) => ("DecodeError: expected no more data, got", b),
            DecodeError::ExpectedString(b) => ("DecodeError: expected string, got", b),
            DecodeError::IncompleteEscape(b) => ("DecodeError: incomplete escape sequence", b),
            DecodeError::InvalidEscape(b) => ("DecodeError: invalid escape sequence", b),
            DecodeError::ListEndNotConsumed(b) => {
                ("DecodeError: program error: list end not consumed, at", b)
            }
            DecodeError::NotInList(b) => ("DecodeError: program error: not in list, at", b),
            DecodeError::NotUtf8(b) => ("DecodeError: expected UTF-8, got", b),
            DecodeError::UnclosedString(b) => ("DecodeError: unclosed string, got", b),
        };
        write!(f, "{prefix}: '{}'", escape_and_elide(b, 40))
    }
}

pub trait Decode {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError>
    where
        Self: Sized;
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut decoder = Decoder::new(bytes);
        Self::decode_using(&mut decoder)
    }
}

pub struct Decoder<'a> {
    bytes: &'a [u8],
    list_depth: usize,
    consumed_list_element: bool,
}
impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            list_depth: 0,
            consumed_list_element: false,
        }
    }

    pub fn close(self) -> Result<(), DecodeError> {
        if self.list_depth != 0 {
            return Err(DecodeError::ListEndNotConsumed(self.debug_vec()));
        }
        if !self.bytes.is_empty() {
            return Err(DecodeError::DataNotConsumed(self.debug_vec()));
        }
        Ok(())
    }

    fn debug_vec(&self) -> Vec<u8> {
        Vec::from_iter(self.bytes.iter().take(21).copied())
    }

    pub fn consume_bool(&mut self) -> Result<bool, DecodeError> {
        let value = match self.bytes.first() {
            Some(b'T') => true,
            Some(b'F') => false,
            _ => return Err(DecodeError::ExpectedBool(self.debug_vec())),
        };
        self.consume_bytes(1);
        self.consume_list_separator()?;
        Ok(value)
    }

    fn consume_bytes(&mut self, n: usize) {
        self.bytes = &self.bytes[n..];
    }

    fn consume_exact(&mut self, c: u8) -> Option<()> {
        if self.bytes.first() == Some(&c) {
            self.bytes = &self.bytes[1..];
            Some(())
        } else {
            None
        }
    }

    pub fn consume_open_list(&mut self) -> Result<(), DecodeError> {
        self.consume_exact(b'[')
            .ok_or_else(|| DecodeError::ExpectedString(self.debug_vec()))?;
        self.list_depth += 1;
        Ok(())
    }

    pub fn consume_list_separator(&mut self) -> Result<(), DecodeError> {
        if self.list_depth == 0 {
            Ok(())
        } else {
            self.consumed_list_element = true;
            match self.bytes.first() {
                Some(&b',') => {
                    self.consume_bytes(1);
                    Ok(())
                }
                Some(&b']') => Ok(()),
                None => Ok(()), // Next call will try to consume list close and fail.
                _ => Err(DecodeError::ExpectedListSeparator(self.debug_vec())),
            }
        }
    }

    pub fn has_another_list_item(&mut self) -> bool {
        match self.bytes.first() {
            None | Some(&b']') => false,
            _ => true,
        }
    }

    pub fn consume_close_list(&mut self) -> Result<(), DecodeError> {
        if self.list_depth == 0 {
            return Err(DecodeError::NotInList(self.debug_vec()));
        }
        self.consume_exact(b']')
            .ok_or_else(|| DecodeError::ExpectedListEnd(self.debug_vec()))?;
        self.consumed_list_element = false;
        self.consume_list_separator()?;
        self.list_depth -= 1;
        Ok(())
    }

    pub fn consume_string(&mut self) -> Result<String, DecodeError> {
        match self.bytes.first() {
            Some(b'"') => {}
            _ => return Err(DecodeError::ExpectedString(self.debug_vec())),
        }
        let Some((len, _)) = self
            .bytes
            .iter()
            .copied()
            .enumerate()
            .skip(1)
            .find(|(_n, b)| b == &b'"')
        else {
            return Err(DecodeError::UnclosedString(self.debug_vec()));
        };
        let s = core::str::from_utf8(&self.bytes[1..len])
            .map_err(|_e| DecodeError::NotUtf8(self.debug_vec()))?;
        let mut value = String::with_capacity(len - 1);
        fn from_hex(c: char) -> Option<u8> {
            match c {
                '0' => Some(0),
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                '5' => Some(5),
                '6' => Some(6),
                '7' => Some(7),
                '8' => Some(8),
                '9' => Some(9),
                'a' => Some(10),
                'b' => Some(11),
                'c' => Some(12),
                'd' => Some(13),
                'e' => Some(14),
                'f' => Some(15),
                _ => None,
            }
        }
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                let opt_c1 = chars.next();
                let opt_c2 = chars.next();
                let (Some(c1), Some(c2)) = (opt_c1, opt_c2) else {
                    return Err(DecodeError::IncompleteEscape(
                        [Some('\\'), opt_c1, opt_c2, Some('"')]
                            .iter()
                            .flatten()
                            .collect::<String>()
                            .into_bytes(),
                    ));
                };
                let (Some(b1), Some(b2)) = (from_hex(c1), from_hex(c2)) else {
                    return Err(DecodeError::InvalidEscape(
                        format!("\\{c1}{c2}").into_bytes(),
                    ));
                };
                let b = 16 * b1 + b2;
                match b {
                    0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | // Allowed.
                    0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | //
                    0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | // 
                    0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f | //
                    0x22 | 0x5c | 0x7f => value.push(char::from(b)),
                    _ => return Err(DecodeError::InvalidEscape(
                        format!("\\{c1}{c2}").into_bytes(),
                    ))
                }
            } else {
                value.push(c);
            }
        }
        self.consume_bytes(len + 1);
        self.consume_list_separator()?;
        Ok(value)
    }
}
