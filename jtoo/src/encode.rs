use std::iter::repeat;
use std::ops::Rem;

#[derive(Debug, Eq, PartialEq)]
pub enum EncodeError {
    NotInByteString,
    NotInList,
    NotInString,
    UnclosedByteString,
    UnclosedList,
    UnclosedString,
    InvalidDecimal,
    InvalidTimestamp,
}

pub trait Encode {
    fn encode_using(&self, packer: &mut Encoder) -> Result<(), EncodeError>;
    fn encode(&self) -> Result<String, EncodeError> {
        let mut encoder = Encoder::new();
        self.encode_using(&mut encoder)?;
        encoder.to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Elem {
    String,
    ByteString,
    EmptyList,
    List,
}

pub struct Encoder {
    stack: Vec<Elem>,
    string: String,
}
impl Encoder {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            string: String::new(),
        }
    }

    fn prepare_for_new_value(&mut self) -> Result<(), EncodeError> {
        match self.stack.last() {
            Some(Elem::String) => Err(EncodeError::UnclosedString),
            Some(Elem::ByteString) => Err(EncodeError::UnclosedByteString),
            Some(Elem::EmptyList) => {
                self.stack.pop();
                self.stack.push(Elem::List);
                Ok(())
            }
            None => Ok(()),
            Some(Elem::List) => {
                self.string.push(',');
                Ok(())
            }
        }
    }

    pub fn append_bool(&mut self, b: bool) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push(if b { 'T' } else { 'F' });
        Ok(())
    }

    pub fn open_byte_string(&mut self) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::ByteString);
        self.string.push('B');
        Ok(())
    }

    pub fn append_byte_string(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        #[inline]
        fn to_hex(b: u8) -> char {
            match b {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => unreachable!(),
            }
        }
        if self.stack.last() == Some(&Elem::ByteString) {
            for b in bytes {
                self.string.push(to_hex(b >> 4));
                self.string.push(to_hex(b & 0x0F));
            }
            Ok(())
        } else {
            Err(EncodeError::NotInString)
        }
    }

    pub fn close_byte_string(&mut self) -> Result<(), EncodeError> {
        if self.stack.last() == Some(&Elem::ByteString) {
            self.stack.pop();
            Ok(())
        } else {
            Err(EncodeError::NotInString)
        }
    }

    pub fn append_decimal(&mut self, value: i64, base10_exponent: i8) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        if value == 0 {
            self.string.push_str("0.0");
            return Ok(());
        }
        if value.is_negative() {
            self.string.push('-')
        }
        let mut digits_array = [0u8; 20];
        let mut digits_len = 0;
        let mut value = value.unsigned_abs();
        while value > 0 {
            let digit = (value % 10) as u8;
            digits_array[digits_len] = digit;
            digits_len += 1;
            value /= 10;
        }
        let rhs_len = base10_exponent.min(0).unsigned_abs() as usize;
        let lhs_len = if base10_exponent < 0 {
            digits_len.saturating_sub(base10_exponent.unsigned_abs() as usize)
        } else {
            digits_len + (base10_exponent.unsigned_abs() as usize)
        };
        if lhs_len == 0 {
            self.string.push_str("0");
        }
        for (n, digit) in repeat(0)
            .take(rhs_len.saturating_sub(digits_len))
            .chain(digits_array.iter().take(digits_len).rev().copied())
            .chain(repeat(0))
            .take(lhs_len + rhs_len)
            .enumerate()
        {
            if (0 < n) && (n < lhs_len) {
                if (lhs_len - n) % 3 == 0 {
                    self.string.push('_')
                }
            } else if n == lhs_len {
                self.string.push('.')
            } else if lhs_len < n {
                if (n - lhs_len) % 3 == 0 {
                    self.string.push('_')
                }
            }
            self.string
                .push(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][digit as usize])
        }
        if 0 <= base10_exponent {
            self.string.push_str(".0");
        }
        Ok(())
    }

    pub fn append_integer(&mut self, value: i64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        let digits = value.unsigned_abs().to_string();
        if value.is_negative() {
            self.string.push('-')
        }
        let mut has_prev = false;
        for (n, c) in digits.chars().enumerate() {
            let k = digits.len() - n;
            if has_prev && k.rem(3) == 0 {
                self.string.push('_')
            }
            self.string.push(c);
            has_prev = true;
        }
        Ok(())
    }

    pub fn open_list(&mut self) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::EmptyList);
        self.string.push('[');
        Ok(())
    }

    pub fn close_list(&mut self) -> Result<(), EncodeError> {
        match self.stack.last() {
            Some(&Elem::EmptyList) | Some(&Elem::List) => {
                self.string.push(']');
                self.stack.pop();
                Ok(())
            }
            _ => Err(EncodeError::NotInList),
        }
    }

    pub fn open_string(&mut self) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::String);
        self.string.push('"');
        Ok(())
    }

    pub fn append_string(&mut self, s: &str) -> Result<(), EncodeError> {
        if self.stack.last() == Some(&Elem::String) {
            self.string.push_str(s);
            Ok(())
        } else {
            Err(EncodeError::NotInString)
        }
    }

    pub fn close_string(&mut self) -> Result<(), EncodeError> {
        if self.stack.last() == Some(&Elem::String) {
            self.stack.pop();
            self.string.push('"');
            Ok(())
        } else {
            Err(EncodeError::NotInString)
        }
    }

    // TODO: Implement times.

    pub fn append_timestamp_seconds(&mut self, s: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(s).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_integer(value)
    }

    pub fn append_timestamp_milliseconds(&mut self, ms: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(ms).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_decimal(value, -3)
    }

    pub fn append_timestamp_microseconds(&mut self, us: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(us).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_decimal(value, -6)
    }

    pub fn append_timestamp_nanosecond(&mut self, ns: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(ns).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_decimal(value, -9)
    }

    pub fn to_string(mut self) -> Result<String, EncodeError> {
        match self.stack.pop() {
            Some(Elem::String) => Err(EncodeError::UnclosedString),
            Some(Elem::ByteString) => Err(EncodeError::UnclosedByteString),
            Some(Elem::EmptyList) | Some(Elem::List) => Err(EncodeError::UnclosedList),
            None => Ok(self.string),
        }
    }
}
