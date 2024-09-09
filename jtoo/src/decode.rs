use crate::escape_ascii;
use core::fmt::Debug;
use safe_regex::regex;

#[derive(Eq, PartialEq)]
pub enum DecodeError {
    DataNotConsumed(Vec<u8>),
    ExpectedBool(Vec<u8>),
    ExpectedInteger(Vec<u8>),
    ExpectedListEnd(Vec<u8>),
    ExpectedListSeparator(Vec<u8>),
    ExpectedNoMoreData(Vec<u8>),
    ExpectedString(Vec<u8>),
    ExtraLeadingZeroes(Vec<u8>),
    IncompleteEscape(Vec<u8>),
    IncorrectDigitGrouping(Vec<u8>),
    InvalidEscape(Vec<u8>),
    ListEndNotConsumed(Vec<u8>),
    NegativeZero(Vec<u8>),
    NotInList(Vec<u8>),
    NotUtf8(Vec<u8>),
    UnclosedString(Vec<u8>),
}
impl Debug for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        let (prefix, b) = match self {
            DecodeError::DataNotConsumed(b) => ("DecodeError: program error: data not consumed", b),
            DecodeError::ExpectedBool(b) => ("DecodeError: expected bool, got", b),
            DecodeError::ExpectedInteger(b) => ("DecodeError: expected integer, got", b),
            DecodeError::ExpectedListEnd(b) => ("DecodeError: expected list end, got", b),
            DecodeError::ExpectedListSeparator(b) => ("DecodeError: expected comma, got", b),
            DecodeError::ExpectedNoMoreData(b) => ("DecodeError: expected no more data, got", b),
            DecodeError::ExpectedString(b) => ("DecodeError: expected string, got", b),
            DecodeError::ExtraLeadingZeroes(b) => ("DecodeError: expected single zero, got", b),
            DecodeError::IncompleteEscape(b) => ("DecodeError: incomplete escape sequence", b),
            DecodeError::IncorrectDigitGrouping(b) => ("DecodeError: incorrect digit grouping", b),
            DecodeError::InvalidEscape(b) => ("DecodeError: invalid escape sequence", b),
            DecodeError::ListEndNotConsumed(b) => {
                ("DecodeError: program error: list end not consumed, at", b)
            }
            DecodeError::NegativeZero(b) => ("DecodeError: got negative zero", b),
            DecodeError::NotInList(b) => ("DecodeError: program error: not in list, at", b),
            DecodeError::NotUtf8(b) => ("DecodeError: expected UTF-8, got", b),
            DecodeError::UnclosedString(b) => ("DecodeError: unclosed string, got", b),
        };
        write!(f, "{prefix}: '{}'", escape_ascii(b))
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

    pub fn consume_integer(&mut self) -> Result<i64, DecodeError> {
        let len = self
            .bytes
            .iter()
            .copied()
            .take_while(|b| match *b {
                b'-' | b'0'..=b'9' | b'_' => true,
                _ => false,
            })
            .count();
        let bytes = &self.bytes[..len];
        // TODO: Update safe-regex is_match to short-circuit .* and not process remaining data.  Then delete `len`.
        if !regex!(br"-?[_0-9]+").is_match(bytes) {
            return Err(DecodeError::ExpectedInteger(self.debug_vec()));
        }
        let (sign, digits) = regex!(br"(-?)([0-9]{1,3}(?:_[0-9]{3})*)(?:[^_0-9].*)?")
            .match_slices(bytes)
            .ok_or_else(|| DecodeError::IncorrectDigitGrouping(self.debug_vec()))?;
        if !regex!(br"-?(?:0|0_?[1-9].*|[1-9].*)").is_match(bytes) {
            return Err(DecodeError::ExtraLeadingZeroes(self.debug_vec()));
        }
        let mut value = 0;
        for b in digits.iter().copied().filter(|b| (b'0'..=b'9').contains(b)) {
            value *= 10;
            value += i64::from(b - b'0');
        }
        if !sign.is_empty() {
            if value == 0 {
                return Err(DecodeError::NegativeZero(self.debug_vec()));
            } else {
                value *= -1
            }
        }
        self.consume_bytes(sign.len() + digits.len());
        self.consume_list_separator()?;
        Ok(value)
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
                let (Some(b1), Some(b2)) = (c1.to_digit(16), c2.to_digit(16)) else {
                    return Err(DecodeError::InvalidEscape(
                        format!("\\{c1}{c2}").into_bytes(),
                    ));
                };
                let b = 16 * b1 + b2;
                match b {
                    0x00..=0x1f | 0x22 | 0x5c | 0x7f => value.push(char::from_u32(b).unwrap()),
                    _ => {
                        return Err(DecodeError::InvalidEscape(
                            format!("\\{c1}{c2}").into_bytes(),
                        ))
                    }
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
