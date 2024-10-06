use std::iter::repeat;
use std::ops::Rem;

#[allow(clippy::module_name_repetitions)]
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
    InvalidYear,
    InvalidMonth,
    InvalidHour,
    InvalidMinute,
    InvalidDay,
    InvalidSecond,
    InvalidWeek,
    InvalidWeekday,
    InvalidNanosecond,
    InvalidMicrosecond,
    InvalidMillisecond,
    InvalidTimezoneOffset,
}

pub trait Encode {
    #[allow(clippy::missing_errors_doc)]
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError>;

    #[allow(clippy::missing_errors_doc)]
    fn encode(&self) -> Result<String, EncodeError> {
        let mut encoder = Encoder::new();
        self.encode_using(&mut encoder)?;
        encoder.into_string()
    }
}

#[derive(Debug)]
pub struct YearAppender<'x>(&'x mut Encoder);
impl<'x> YearAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_month(self, month: u8) -> Result<MonthAppender<'x>, EncodeError> {
        self.0.append_month(month)?;
        Ok(MonthAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_week(self, week: u8) -> Result<WeekAppender<'x>, EncodeError> {
        self.0.append_week(week)?;
        Ok(WeekAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct MonthAppender<'x>(&'x mut Encoder);
impl<'x> MonthAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_day(self, day: u8) -> Result<DayAppender<'x>, EncodeError> {
        self.0.append_day(day)?;
        Ok(DayAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct WeekAppender<'x>(&'x mut Encoder);
impl<'x> WeekAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_weekday(self, weekday: u8) -> Result<DayAppender<'x>, EncodeError> {
        self.0.append_weekday(weekday)?;
        Ok(DayAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct DayAppender<'x>(&'x mut Encoder);
impl<'x> DayAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_hour(self, hour: u8) -> Result<HourAppender<'x>, EncodeError> {
        self.0.append_hour_internal(hour)?;
        Ok(HourAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct HourAppender<'x>(&'x mut Encoder);
impl<'x> HourAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_minute(self, minute: u8) -> Result<MinuteAppender<'x>, EncodeError> {
        self.0.append_minute(minute)?;
        Ok(MinuteAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct MinuteAppender<'x>(&'x mut Encoder);
impl<'x> MinuteAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_second(self, second: u8) -> Result<SecondAppender<'x>, EncodeError> {
        self.0.append_second(second)?;
        Ok(SecondAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_millisecond(self, millisecond: u32) -> Result<SecondAppender<'x>, EncodeError> {
        self.0.append_millisecond(millisecond)?;
        Ok(SecondAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_microsecond(self, microsecond: u32) -> Result<SecondAppender<'x>, EncodeError> {
        self.0.append_microsecond(microsecond)?;
        Ok(SecondAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_nanosecond(self, nanosecond: u64) -> Result<SecondAppender<'x>, EncodeError> {
        self.0.append_nanosecond(nanosecond)?;
        Ok(SecondAppender(self.0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct SecondAppender<'x>(&'x mut Encoder);
impl<'x> SecondAppender<'x> {
    #[allow(clippy::missing_errors_doc)]
    pub fn append_tzoffset(self, hour: i8, minute: u8) -> Result<&'x mut Encoder, EncodeError> {
        self.0.append_tzoffset(hour, minute)?;
        Ok(self.0)
    }
}

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

    fn append_2_digits(&mut self, value: u8) {
        let d1 = value % 10;
        let d0 = value / 10;
        self.string.push(Self::DIGITS[d0 as usize]);
        self.string.push(Self::DIGITS[d1 as usize]);
    }

    pub(crate) fn append_month(&mut self, month: u8) -> Result<(), EncodeError> {
        if !(1..=12).contains(&month) {
            return Err(EncodeError::InvalidMonth);
        }
        self.string.push('-');
        self.append_2_digits(month);
        Ok(())
    }

    pub(crate) fn append_day(&mut self, day: u8) -> Result<(), EncodeError> {
        if !(1..=31).contains(&day) {
            return Err(EncodeError::InvalidDay);
        }
        self.string.push('-');
        self.append_2_digits(day);
        Ok(())
    }

    pub(crate) fn append_week(&mut self, week: u8) -> Result<(), EncodeError> {
        if !(1..=53).contains(&week) {
            return Err(EncodeError::InvalidWeek);
        }
        self.string.push('-');
        self.string.push('W');
        self.append_2_digits(week);
        Ok(())
    }

    pub(crate) fn append_weekday(&mut self, weekday: u8) -> Result<(), EncodeError> {
        if !(1..=7).contains(&weekday) {
            return Err(EncodeError::InvalidWeekday);
        }
        self.string.push('-');
        self.append_2_digits(weekday);
        Ok(())
    }

    pub(crate) fn append_hour_internal(&mut self, hour: u8) -> Result<(), EncodeError> {
        if 23 < hour {
            return Err(EncodeError::InvalidHour);
        }
        self.string.push('T');
        self.append_2_digits(hour);
        Ok(())
    }

    pub(crate) fn append_minute(&mut self, minute: u8) -> Result<(), EncodeError> {
        if 59 < minute {
            return Err(EncodeError::InvalidMinute);
        }
        self.string.push(':');
        self.append_2_digits(minute);
        Ok(())
    }

    pub(crate) fn append_second(&mut self, second: u8) -> Result<(), EncodeError> {
        if 60 < second {
            return Err(EncodeError::InvalidSecond);
        }
        self.string.push(':');
        self.append_2_digits(second);
        Ok(())
    }

    pub(crate) fn append_millisecond(&mut self, millisecond: u32) -> Result<(), EncodeError> {
        if 60_999 < millisecond {
            return Err(EncodeError::InvalidMillisecond);
        }
        self.string.push(':');
        if millisecond < 10_000 {
            self.string.push('0');
        }
        self.append_decimal(i64::from(millisecond), -3)
    }

    pub(crate) fn append_microsecond(&mut self, microsecond: u32) -> Result<(), EncodeError> {
        if 60_999_999 < microsecond {
            return Err(EncodeError::InvalidMicrosecond);
        }
        self.string.push(':');
        if microsecond < 10_000_000 {
            self.string.push('0');
        }
        self.append_decimal(i64::from(microsecond), -6)
    }

    pub(crate) fn append_nanosecond(&mut self, nanosecond: u64) -> Result<(), EncodeError> {
        if 60_999_999_999 < nanosecond {
            return Err(EncodeError::InvalidNanosecond);
        }
        let value = i64::try_from(nanosecond).unwrap();
        self.string.push(':');
        if nanosecond < 10_000_000_000 {
            self.string.push('0');
        }
        self.append_decimal(value, -9)
    }

    pub(crate) fn append_tzoffset(&mut self, hour: i8, minute: u8) -> Result<(), EncodeError> {
        if !(-12..=12).contains(&hour) || !(0..=59).contains(&minute) {
            return Err(EncodeError::InvalidTimezoneOffset);
        }
        if hour == 0 && minute == 0 {
            self.string.push('Z');
            return Ok(());
        }
        self.string.push(if -1 < hour { '+' } else { '-' });
        self.append_2_digits(hour.unsigned_abs());
        if minute > 0 {
            self.append_2_digits(minute);
        }
        Ok(())
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
    pub fn append_year(&mut self, mut year: u16) -> Result<YearAppender, EncodeError> {
        self.prepare_for_new_value()?;
        if !(1..=9999).contains(&year) {
            return Err(EncodeError::InvalidYear);
        }
        self.string.push('D');
        let d3 = year % 10;
        year /= 10;
        let d2 = year % 10;
        year /= 10;
        let d1 = year % 10;
        let d0 = year / 10;
        if d0 != 0 {
            self.string.push(Self::DIGITS[d0 as usize]);
        }
        if d0 != 0 || d1 != 0 {
            self.string.push(Self::DIGITS[d1 as usize]);
        }
        if d0 != 0 || d1 != 0 || d2 != 0 {
            self.string.push(Self::DIGITS[d2 as usize]);
        }
        self.string.push(Self::DIGITS[d3 as usize]);
        Ok(YearAppender(self))
    }

    /// `T23`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_hour(&mut self, hour: u8) -> Result<HourAppender, EncodeError> {
        self.prepare_for_new_value()?;
        self.append_hour_internal(hour)?;
        Ok(HourAppender(self))
    }

    /// `1.0`, `1_234.567_8`, `0.01`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_decimal(&mut self, value: i64, base10_exponent: i8) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
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

    /// `1_234`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_integer(&mut self, value: i64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
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
    pub fn append_string(&mut self, s: &str) -> Result<(), EncodeError> {
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
        self.append_integer(value)
    }

    /// `S1_234.500`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_milliseconds(&mut self, ms: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(ms).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_decimal(value, -3)
    }

    /// `S1_234.567_800`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_microseconds(&mut self, us: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(us).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_decimal(value, -6)
    }

    /// `S1_234.567_890_100`
    #[allow(clippy::missing_errors_doc)]
    pub fn append_timestamp_nanosecond(&mut self, ns: u64) -> Result<(), EncodeError> {
        self.prepare_for_new_value()?;
        self.string.push('S');
        let value = i64::try_from(ns).map_err(|_| EncodeError::InvalidTimestamp)?;
        self.append_decimal(value, -9)
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
            None => Ok(self.string.as_str()),
        }
    }
}
