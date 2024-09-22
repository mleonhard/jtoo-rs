use crate::escape_ascii;
use core::fmt::Debug;

trait AsDecimalDigit {
    fn to_decimal_digit(&self) -> Option<u8>;
}
impl AsDecimalDigit for Option<u8> {
    fn to_decimal_digit(&self) -> Option<u8> {
        match self {
            Some(b) if (b'0'..=b'9').contains(b) => Some(*b - b'0'),
            _ => None,
        }
    }
}

trait ByteIterExt: Iterator<Item = u8> {
    fn next_decimal_digit(&mut self) -> Option<u8>;
}
impl<T> ByteIterExt for T
where
    T: Iterator<Item = u8>,
{
    fn next_decimal_digit(&mut self) -> Option<u8> {
        match self.next() {
            Some(b'0') => Some(0),
            Some(b'1') => Some(1),
            Some(b'2') => Some(2),
            Some(b'3') => Some(3),
            Some(b'4') => Some(4),
            Some(b'5') => Some(5),
            Some(b'6') => Some(6),
            Some(b'7') => Some(7),
            Some(b'8') => Some(8),
            Some(b'9') => Some(9),
            _ => None,
        }
    }
}

// TODO: Turn this into struct DecodeError(Reason,usize, &[u8])
#[derive(Eq, PartialEq)]
pub enum DecodeError {
    DataNotConsumed(Vec<u8>),
    DayOutOfRange(Vec<u8>),
    ExpectedBool(Vec<u8>),
    ExpectedDate(Vec<u8>),
    ExpectedDay(Vec<u8>),
    ExpectedInteger(Vec<u8>),
    ExpectedListEnd(Vec<u8>),
    ExpectedListSeparator(Vec<u8>),
    ExpectedMonth(Vec<u8>),
    ExpectedNoMoreData(Vec<u8>),
    ExpectedString(Vec<u8>),
    ExpectedWeek(Vec<u8>),
    ExtraLeadingZeroes(Vec<u8>),
    IncompleteEscape(Vec<u8>),
    IncorrectDigitGrouping(Vec<u8>),
    IntegerTooLarge(Vec<u8>),
    InvalidEscape(Vec<u8>),
    ListEndNotConsumed(Vec<u8>),
    MalformedDate(Vec<u8>),
    MonthOutOfRange(Vec<u8>),
    NegativeZero(Vec<u8>),
    NotInList(Vec<u8>),
    NotUtf8(Vec<u8>),
    UnclosedString(Vec<u8>),
    WeekOutOfRange(Vec<u8>),
    YearOutOfRange(Vec<u8>),
}
impl Debug for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        let (prefix, b) = match self {
            DecodeError::DataNotConsumed(b) => ("DecodeError: program error: data not consumed", b),
            DecodeError::DayOutOfRange(b) => ("DecodeError: day out of range", b),
            DecodeError::ExpectedBool(b) => ("DecodeError: expected bool, got", b),
            DecodeError::ExpectedDate(b) => ("DecodeError: expected date, got", b),
            DecodeError::ExpectedDay(b) => ("DecodeError: expected day, got", b),
            DecodeError::ExpectedInteger(b) => ("DecodeError: expected integer, got", b),
            DecodeError::ExpectedListEnd(b) => ("DecodeError: expected list end, got", b),
            DecodeError::ExpectedListSeparator(b) => ("DecodeError: expected comma, got", b),
            DecodeError::ExpectedMonth(b) => ("DecodeError: expected month, got", b),
            DecodeError::ExpectedNoMoreData(b) => ("DecodeError: expected no more data, got", b),
            DecodeError::ExpectedString(b) => ("DecodeError: expected string, got", b),
            DecodeError::ExpectedWeek(b) => ("DecodeError: expected week, got", b),
            DecodeError::ExtraLeadingZeroes(b) => ("DecodeError: expected single zero, got", b),
            DecodeError::IncompleteEscape(b) => ("DecodeError: incomplete escape sequence", b),
            DecodeError::IncorrectDigitGrouping(b) => ("DecodeError: incorrect digit grouping", b),
            DecodeError::IntegerTooLarge(b) => ("DecodeError: integer is too large", b),
            DecodeError::InvalidEscape(b) => ("DecodeError: invalid escape sequence", b),
            DecodeError::ListEndNotConsumed(b) => {
                ("DecodeError: program error: list end not consumed, at", b)
            }
            DecodeError::MalformedDate(b) => ("DecodeError: got malformed date", b),
            DecodeError::MonthOutOfRange(b) => ("DecodeError: month out of range", b),
            DecodeError::NegativeZero(b) => ("DecodeError: got negative zero", b),
            DecodeError::NotInList(b) => ("DecodeError: program error: not in list, at", b),
            DecodeError::NotUtf8(b) => ("DecodeError: expected UTF-8, got", b),
            DecodeError::UnclosedString(b) => ("DecodeError: unclosed string, got", b),
            DecodeError::WeekOutOfRange(b) => ("DecodeError: week out of range", b),
            DecodeError::YearOutOfRange(b) => ("DecodeError: year out of range", b),
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

#[derive(Debug)]
pub struct Date {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub week: Option<u8>,
    pub day: Option<u8>,
}

#[derive(Debug)]
pub enum Second {
    Integer(u8),
    Milli(u32),
    Micro(u32),
    Nano(u64),
}

#[derive(Debug)]
pub struct Time {
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Second,
}

#[derive(Debug)]
pub struct TzOffset {
    pub hours: i8,
    pub minutes: u8, // Only one country has used a -00xx timezone, Liberia.  They stopped in 1972.
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Elem {
    None,
    Year,
    MonthOrWeek,
    Day,
    Hour,
    Minute,
    Second,
}

#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    debug_bytes: &'a [u8],
    consumed_list_element: bool,
    list_depth: usize,
    previous: Elem,
}
impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            debug_bytes: bytes,
            consumed_list_element: false,
            list_depth: 0,
            previous: Elem::None,
        }
    }

    pub fn expect_previous(&self, expected: Elem) -> Result<(), DecodeError> {
        if self.previous == expected {
            Ok(())
        } else {
            Err(match expected {
                Elem::None => DecodeError::DataNotConsumed(self.debug_vec()),
                Elem::Year
                | Elem::MonthOrWeek
                | Elem::Day
                | Elem::Hour
                | Elem::Minute
                | Elem::Second => DecodeError::MalformedDate(self.debug_vec()),
            })
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
        Vec::from_iter(self.debug_bytes.iter().take(30).copied())
    }

    pub fn consume_bool(&mut self) -> Result<bool, DecodeError> {
        self.expect_previous(Elem::None)?;
        let value = match self.consume_byte() {
            Some(b'T') => true,
            Some(b'F') => false,
            _ => return Err(DecodeError::ExpectedBool(self.debug_vec())),
        };
        self.close_item()?;
        Ok(value)
    }

    fn consume_byte(&mut self) -> Option<u8> {
        match self.bytes.get(0).copied() {
            Some(b) => {
                self.bytes = &self.bytes[1..];
                Some(b)
            }
            None => None,
        }
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
        self.expect_previous(Elem::None)?;
        let mut sign = if self.consume_exact(b'-').is_some() {
            -1
        } else {
            1
        };
        let mut len = 0;
        let mut seen_underscore = false;
        let mut group_digit_count = 0u16;
        let mut value = 0i64;
        while let Some(b) = self.consume_byte() {
            match b {
                b'-' => {
                    if len == 0 {
                        sign = -1;
                    } else {
                        return Err(DecodeError::ExpectedInteger(self.debug_vec()));
                    }
                }
                b'0'..=b'9' => {
                    group_digit_count += 1;
                    let d = i64::from(b - b'0');
                    value = value
                        .checked_mul(10)
                        .ok_or_else(|| DecodeError::IntegerTooLarge(self.debug_vec()))?;
                    value = value
                        .checked_add(sign * d)
                        .ok_or_else(|| DecodeError::IntegerTooLarge(self.debug_vec()))?;
                    if value == 0 && 1 < group_digit_count {
                        return Err(DecodeError::ExtraLeadingZeroes(self.debug_vec()));
                    }
                }
                b'_' => {
                    if seen_underscore {
                        if group_digit_count != 3 {
                            return Err(DecodeError::IncorrectDigitGrouping(self.debug_vec()));
                        }
                    } else {
                        if value == 0 {
                            return Err(DecodeError::IncorrectDigitGrouping(self.debug_vec()));
                        }
                    }
                    seen_underscore = true;
                    group_digit_count = 0;
                }
                _ => break,
            }
            len += 1;
        }
        if value == 0 && group_digit_count == 0 {
            return Err(DecodeError::ExpectedInteger(self.debug_vec()));
        }
        if seen_underscore && group_digit_count != 3 {
            return Err(DecodeError::IncorrectDigitGrouping(self.debug_vec()));
        }
        if !seen_underscore && 3 < group_digit_count {
            return Err(DecodeError::IncorrectDigitGrouping(self.debug_vec()));
        }
        if sign == -1 && value == 0 {
            return Err(DecodeError::NegativeZero(self.debug_vec()));
        }
        self.close_item()?;
        Ok(value)
    }

    pub fn consume_open_list(&mut self) -> Result<(), DecodeError> {
        self.expect_previous(Elem::None)?;
        self.consume_exact(b'[')
            .ok_or_else(|| DecodeError::ExpectedString(self.debug_vec()))?;
        self.list_depth += 1;
        Ok(())
    }

    pub fn close_item(&mut self) -> Result<(), DecodeError> {
        let result = if self.list_depth == 0 {
            Ok(())
        } else {
            self.consumed_list_element = true;
            match self.bytes.first() {
                Some(&b',') => {
                    self.consume_byte();
                    Ok(())
                }
                Some(&b']') => Ok(()),
                None => Ok(()), // Next call will try to consume list close and fail.
                _ => Err(DecodeError::ExpectedListSeparator(self.debug_vec())),
            }
        };
        self.debug_bytes = self.bytes;
        result
    }

    pub fn maybe_end_date(&mut self) -> Result<(), DecodeError> {
        match (self.previous, self.bytes.get(0).copied()) {
            (_, None) => Ok(()),
            (_, Some(b']') | Some(b',')) => {
                self.previous = Elem::None;
                self.close_item()
            }
            (Elem::Year | Elem::MonthOrWeek, Some(b'-') | Some(b'~') | Some(b'+') | Some(b'Z'))
            | (Elem::Day, Some(b'T') | Some(b'~') | Some(b'+') | Some(b'Z'))
            | (Elem::Hour | Elem::Minute, Some(b':') | Some(b'~') | Some(b'+') | Some(b'Z'))
            | (Elem::Second, Some(b'~') | Some(b'+') | Some(b'Z')) => Ok(()),
            _ => Err(DecodeError::MalformedDate(self.debug_vec())),
        }
    }

    pub fn consume_date_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if (b'0'..=b'9').contains(&b) => Ok(b - b'0'),
            _ => Err(DecodeError::MalformedDate(self.debug_vec())),
        }
    }

    pub fn has_more(&self) -> bool {
        self.previous != Elem::None
    }

    pub fn consume_year(&mut self) -> Result<u16, DecodeError> {
        self.expect_previous(Elem::None)?;
        if self.consume_byte() != Some(b'D') {
            return Err(DecodeError::ExpectedDate(self.debug_vec()));
        }
        let d0 = u16::from(self.consume_date_digit()?);
        let d1 = u16::from(self.consume_date_digit()?);
        let d2 = u16::from(self.consume_date_digit()?);
        let d3 = u16::from(self.consume_date_digit()?);
        let year = 1000 * d0 + 100 * d1 + 10 * d2 + d3;
        if !(1..=9999).contains(&year) {
            return Err(DecodeError::YearOutOfRange(self.debug_vec()));
        }
        self.previous = Elem::Year;
        self.maybe_end_date()?;
        Ok(year)
    }

    pub fn consume_month(&mut self) -> Result<u8, DecodeError> {
        self.expect_previous(Elem::Year)?;
        if self.consume_byte() != Some(b'-') {
            return Err(DecodeError::ExpectedMonth(self.debug_vec()));
        }
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let month = 10 * d0 + d1;
        if !(1..=12).contains(&month) {
            return Err(DecodeError::MonthOutOfRange(self.debug_vec()));
        }
        self.previous = Elem::MonthOrWeek;
        self.maybe_end_date()?;
        Ok(month)
    }

    pub fn consume_week(&mut self) -> Result<u8, DecodeError> {
        self.expect_previous(Elem::Year)?;
        if self.consume_byte() != Some(b'-') {
            return Err(DecodeError::ExpectedWeek(self.debug_vec()));
        }
        match self.consume_byte() {
            Some(b'W') => {}
            Some(b) if (b'0'..=b'9').contains(&b) => {
                return Err(DecodeError::ExpectedWeek(self.debug_vec()))
            }
            _ => return Err(DecodeError::MalformedDate(self.debug_vec())),
        }
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let week = 10 * d0 + d1;
        if !(1..=53).contains(&week) {
            return Err(DecodeError::WeekOutOfRange(self.debug_vec()));
        }
        self.previous = Elem::MonthOrWeek;
        self.maybe_end_date()?;
        Ok(week)
    }

    pub fn consume_day(&mut self) -> Result<u8, DecodeError> {
        self.expect_previous(Elem::MonthOrWeek)?;
        if self.consume_byte() != Some(b'-') {
            return Err(DecodeError::ExpectedDay(self.debug_vec()));
        }
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let day = 10 * d0 + d1;
        if !(1..=31).contains(&day) {
            return Err(DecodeError::DayOutOfRange(self.debug_vec()));
        }
        self.previous = Elem::Day;
        self.maybe_end_date()?;
        Ok(day)
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
        self.close_item()?;
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
        self.close_item()?;
        Ok(value)
    }
}
