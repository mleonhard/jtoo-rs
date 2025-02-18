use crate::EncodeError;
use std::iter::repeat;
use std::ops::Rem;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Elem {
    String,
    ByteString,
    EmptyList,
    List,
}

#[derive(Debug, Default)]
pub struct Encoder {
    stack: Vec<Elem>,
    string: String,
}

impl Encoder {
    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    #[must_use]
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

    fn append_2_digits(&mut self, mut value: u8) {
        let d1 = value % 10;
        value /= 10;
        let d0 = value;
        self.string.push(Self::DIGITS[d0 as usize]);
        self.string.push(Self::DIGITS[d1 as usize]);
    }

    fn append_3_digits(&mut self, mut value: u16) {
        let d2 = value % 10;
        value /= 10;
        let d1 = value % 10;
        value /= 10;
        let d0 = value;
        self.string.push(Self::DIGITS[d0 as usize]);
        self.string.push(Self::DIGITS[d1 as usize]);
        self.string.push(Self::DIGITS[d2 as usize]);
    }

    fn append_4_digits(&mut self, mut value: u16) {
        let d3 = value % 10;
        value /= 10;
        let d2 = value % 10;
        value /= 10;
        let d1 = value % 10;
        value /= 10;
        let d0 = value;
        self.string.push(Self::DIGITS[d0 as usize]);
        self.string.push(Self::DIGITS[d1 as usize]);
        self.string.push(Self::DIGITS[d2 as usize]);
        self.string.push(Self::DIGITS[d3 as usize]);
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_bool(&mut self, b: bool) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push(if b { 'T' } else { 'F' });
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn open_byte_string(&mut self) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::ByteString);
        self.string.push('B');
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn append_byte_string(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        if self.stack.last() == Some(&Elem::ByteString) {
            for b in bytes {
                let d1 = u32::from(b >> 4);
                let d2 = u32::from(b & 0x0F);
                let c1 = char::from_digit(d1, 16).unwrap();
                let c2 = char::from_digit(d2, 16).unwrap();
                self.string.push(c1);
                self.string.push(c2);
            }
            Ok(())
        } else {
            Err(EncodeError::NotInByteString)
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn close_byte_string(&mut self) -> Result<(), EncodeError> {
        if self.stack.last() == Some(&Elem::ByteString) {
            self.stack.pop();
            Ok(())
        } else {
            Err(EncodeError::NotInByteString)
        }
    }

    /// `D2023`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_date_time_offset(
        &mut self,
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u64,
        offset_hour: i8,
        offset_minute: u8,
    ) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        if !(1..=9999).contains(&year) {
            return Err(EncodeError::InvalidYear);
        }
        if !(1..=12).contains(&month) {
            return Err(EncodeError::InvalidMonth);
        }
        if !(1..=31).contains(&day) {
            return Err(EncodeError::InvalidDay);
        }
        if 23 < hour {
            return Err(EncodeError::InvalidHour);
        }
        if 59 < minute {
            return Err(EncodeError::InvalidMinute);
        }
        if 60 < second {
            return Err(EncodeError::InvalidSecond);
        }
        if 999_999_999 < nanosecond {
            return Err(EncodeError::InvalidNanosecond);
        }
        if !(-23..=23).contains(&offset_hour) || !(0..=59).contains(&offset_minute) {
            return Err(EncodeError::InvalidOffset);
        }
        self.string.push('D');
        self.append_4_digits(year);
        self.string.push('-');
        self.append_2_digits(month);
        self.string.push('-');
        self.append_2_digits(day);
        self.string.push('T');
        self.append_2_digits(hour);
        self.string.push(':');
        self.append_2_digits(minute);
        self.string.push(':');
        self.append_2_digits(second);
        let millisecond_part = u16::try_from((nanosecond / 1_000_000) % 1000).unwrap();
        let microsecond_part = u16::try_from((nanosecond / 1_000) % 1000).unwrap();
        let nanosecond_part = u16::try_from(nanosecond % 1_000).unwrap();
        if nanosecond_part != 0 {
            self.string.push('.');
            self.append_3_digits(millisecond_part);
            self.string.push('_');
            self.append_3_digits(microsecond_part);
            self.string.push('_');
            self.append_3_digits(nanosecond_part);
        } else if microsecond_part != 0 {
            self.string.push('.');
            self.append_3_digits(millisecond_part);
            self.string.push('_');
            self.append_3_digits(microsecond_part);
        } else if millisecond_part != 0 {
            self.string.push('.');
            self.append_3_digits(millisecond_part);
        }
        if offset_hour == 0 && offset_minute == 0 {
            self.string.push('Z');
        } else {
            self.string.push(if -1 < offset_hour { '+' } else { '-' });
            self.append_2_digits(offset_hour.unsigned_abs());
            if offset_minute > 0 {
                self.append_2_digits(offset_minute);
            }
        }
        Ok(())
    }

    fn push_decimal(&mut self, value: i64, base10_exponent: i8) -> Result<(), EncodeError> {
        if value == 0 {
            if -1 < base10_exponent {
                self.string.push_str("0.0");
            } else {
                self.string.push_str("0.");
                for n in 0..base10_exponent.unsigned_abs() {
                    if 0 < n && (n % 3) == 0 {
                        self.string.push('_');
                    }
                    self.string.push('0');
                }
            }
            return Ok(());
        }
        if value.is_negative() {
            self.string.push('-');
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
            self.string.push('0');
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
                    self.string.push('_');
                }
            } else if n == lhs_len {
                self.string.push('.');
            } else if lhs_len < n && (n - lhs_len) % 3 == 0 {
                self.string.push('_');
            }
            self.string.push(Self::DIGITS[digit as usize]);
        }
        if 0 <= base10_exponent {
            self.string.push_str(".0");
        }
        Ok(())
    }

    /// `1.0`, `1_234.567_8`, `0.01`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_decimal(&mut self, value: i64, base10_exponent: i8) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.push_decimal(value, base10_exponent)
    }

    fn push_integer(&mut self, value: i64) -> Result<(), EncodeError> {
        let digits = value.unsigned_abs().to_string();
        if value.is_negative() {
            self.string.push('-');
        }
        let mut has_prev = false;
        for (n, c) in digits.chars().enumerate() {
            let k = digits.len() - n;
            if has_prev && k.rem(3) == 0 {
                self.string.push('_');
            }
            self.string.push(c);
            has_prev = true;
        }
        Ok(())
    }

    /// `1_234`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_integer(&mut self, value: i64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.push_integer(value)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn open_list(&mut self) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::EmptyList);
        self.string.push('[');
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn close_list(&mut self) -> Result<(), EncodeError> {
        match self.stack.last() {
            Some(&Elem::EmptyList | &Elem::List) => {
                self.string.push(']');
                self.stack.pop();
                Ok(())
            }
            _ => Err(EncodeError::NotInList),
        }
    }

    /// `"$s"`
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn append_string(&mut self, s: &str) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('"');
        for c in s.chars() {
            let codepoint = c as u32;
            match codepoint {
                0x00..=0x1f | 0x22 | 0x5c | 0x7f => {
                    let d1 = codepoint >> 4;
                    let d2 = codepoint & 0x0F;
                    let c1 = char::from_digit(d1, 16).unwrap();
                    let c2 = char::from_digit(d2, 16).unwrap();
                    self.string.push('\\');
                    self.string.push(c1);
                    self.string.push(c2);
                }
                _ => self.string.push(c),
            }
        }
        self.string.push('"');
        Ok(())
    }

    /// `"`
    #[allow(clippy::missing_errors_doc)]
    pub fn open_string(&mut self) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::String);
        self.string.push('"');
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn append_open_string(&mut self, s: &str) -> Result<(), EncodeError> {
        if self.stack.last() != Some(&Elem::String) {
            return Err(EncodeError::NotInString);
        }
        for c in s.chars() {
            let codepoint = c as u32;
            match codepoint {
                0x00..=0x1f | 0x22 | 0x5c | 0x7f => {
                    let d1 = codepoint >> 4;
                    let d2 = codepoint & 0x0F;
                    let c1 = char::from_digit(d1, 16).unwrap();
                    let c2 = char::from_digit(d2, 16).unwrap();
                    self.string.push('\\');
                    self.string.push(c1);
                    self.string.push(c2);
                }
                _ => self.string.push(c),
            }
        }
        Ok(())
    }

    /// `"`
    #[allow(clippy::missing_errors_doc)]
    pub fn close_string(&mut self) -> Result<(), EncodeError> {
        if self.stack.last() == Some(&Elem::String) {
            self.stack.pop();
            self.string.push('"');
            Ok(())
        } else {
            Err(EncodeError::NotInString)
        }
    }

    /// `S1_234`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_seconds(&mut self, s: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(s).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.push_integer(value)
    }

    /// `S1_234.500`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_milliseconds(&mut self, ms: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(ms).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.push_decimal(value, -3)
    }

    /// `S1_234.567_800`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_microseconds(&mut self, us: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(us).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.push_decimal(value, -6)
    }

    /// `S1_234.567_890_100`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_nanosecond(&mut self, ns: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(ns).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.push_decimal(value, -9)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn into_string(self) -> Result<String, EncodeError> {
        match self.stack.last() {
            Some(Elem::String) => Err(EncodeError::UnclosedString),
            Some(Elem::ByteString) => Err(EncodeError::UnclosedByteString),
            Some(Elem::EmptyList | Elem::List) => Err(EncodeError::UnclosedList),
            None => Ok(self.string),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn as_str(&self) -> Result<&str, EncodeError> {
        match self.stack.last() {
            Some(Elem::String) => Err(EncodeError::UnclosedString),
            Some(Elem::ByteString) => Err(EncodeError::UnclosedByteString),
            Some(Elem::EmptyList | Elem::List) => Err(EncodeError::UnclosedList),
            None if self.string.is_empty() => Err(EncodeError::Empty),
            None => Ok(self.string.as_str()),
        }
    }
}
