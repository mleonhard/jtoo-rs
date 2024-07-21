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
        if value.is_negative() {
            self.string.push('-')
        }
        let digits = value.unsigned_abs().to_string();
        struct Lengths {
            lhs_len: usize,
            rhs_zeroes: usize,
            rhs_len: usize,
        }
        let lengths = if base10_exponent < 0 {
            let rhs_len = base10_exponent.unsigned_abs() as usize;
            let rhs_zeroes = (rhs_len as i32 - digits.len() as i32).max(0).unsigned_abs() as usize;
            let lhs_len = (digits.len() as i32 - rhs_len as i32).max(0).unsigned_abs() as usize;
            Lengths {
                lhs_len,
                rhs_zeroes,
                rhs_len,
            }
        } else if base10_exponent == 0 {
            Lengths {
                lhs_len: digits.len(),
                rhs_zeroes: 0,
                rhs_len: 0,
            }
        } else {
            let lhs_zeroes = base10_exponent.unsigned_abs() as usize;
            let lhs_len = if value == 0 {
                0
            } else {
                digits.len() + lhs_zeroes
            };
            Lengths {
                lhs_len,
                rhs_zeroes: 0,
                rhs_len: 0,
            }
        };
        if lengths.lhs_len == 0 {
            self.string.push('0')
        }
        let mut has_prev = false;
        for (n, c) in digits
            .chars()
            .chain(repeat('0'))
            .take(lengths.lhs_len)
            .enumerate()
        {
            let k = lengths.lhs_len - n;
            if has_prev && k.rem(3) == 0 {
                self.string.push('_');
            }
            self.string.push(c);
            has_prev = true;
        }
        self.string.push('.');
        let mut has_prev = false;
        for (n, c) in repeat('0')
            .take(lengths.rhs_zeroes)
            .chain(digits.chars().skip(lengths.lhs_len))
            .take(lengths.rhs_len)
            .enumerate()
        {
            if has_prev && n.rem(3) == 0 {
                self.string.push('_')
            }
            self.string.push(c);
            has_prev = true;
        }
        if lengths.rhs_len == 0 {
            self.string.push_str("0")
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

    pub fn to_string(mut self) -> Result<String, EncodeError> {
        match self.stack.pop() {
            Some(Elem::String) => Err(EncodeError::UnclosedString),
            Some(Elem::ByteString) => Err(EncodeError::UnclosedByteString),
            Some(Elem::EmptyList) | Some(Elem::List) => Err(EncodeError::UnclosedList),
            None => Ok(self.string),
        }
    }
}
