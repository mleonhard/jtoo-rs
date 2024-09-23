use crate::escape_ascii;
use core::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorReason {
    DataNotConsumed,
    DayOutOfRange,
    ExpectedBool,
    ExpectedDate,
    ExpectedDateOrTime,
    ExpectedInteger,
    ExpectedListEnd,
    ExpectedListSeparator,
    ExpectedNoMoreData,
    ExpectedSingleZero,
    ExpectedString,
    ExpectedYear,
    ExpectedYearMonth,
    ExpectedYearMonthDay,
    ExpectedYearMonthDayHour,
    ExpectedYearMonthDayHourMinute,
    ExpectedYearMonthDayHourMinuteMicrosecond,
    ExpectedYearMonthDayHourMinuteMicrosecondTzOffset,
    ExpectedYearMonthDayHourMinuteMillisecond,
    ExpectedYearMonthDayHourMinuteMillisecondTzOffset,
    ExpectedYearMonthDayHourMinuteNanosecond,
    ExpectedYearMonthDayHourMinuteNanosecondTzOffset,
    ExpectedYearMonthDayHourMinuteSecond,
    ExpectedYearMonthDayHourMinuteSecondTzOffset,
    ExpectedYearMonthDayHourMinuteTzOffset,
    ExpectedYearMonthDayHourTzOffset,
    ExpectedYearMonthDayTzOffset,
    ExpectedYearMonthTzOffset,
    ExpectedYearTzOffset,
    ExpectedYearWeek,
    ExpectedYearWeekDay,
    ExpectedYearWeekDayHour,
    ExpectedYearWeekDayHourMinute,
    ExpectedYearWeekDayHourMinuteMicrosecond,
    ExpectedYearWeekDayHourMinuteMicrosecondTzOffset,
    ExpectedYearWeekDayHourMinuteMillisecond,
    ExpectedYearWeekDayHourMinuteMillisecondTzOffset,
    ExpectedYearWeekDayHourMinuteNanosecond,
    ExpectedYearWeekDayHourMinuteNanosecondTzOffset,
    ExpectedYearWeekDayHourMinuteSecond,
    ExpectedYearWeekDayHourMinuteSecondTzOffset,
    ExpectedYearWeekDayHourMinuteTzOffset,
    ExpectedYearWeekDayHourTzOffset,
    ExpectedYearWeekDayTzOffset,
    ExpectedYearWeekTzOffset,
    IncompleteEscapeSequence,
    IncorrectDigitGrouping,
    IntegerTooLarge,
    InvalidEscapeSequence,
    ListEndNotConsumed,
    MalformedDateTimeTzOffset,
    MonthOutOfRange,
    NegativeZero,
    NotInList,
    NotUtf8,
    UnclosedString,
    WeekOutOfRange,
    YearOutOfRange,
    Unimplemented,
    ExpectedTime,
    HourOutOfRange,
    MalformedTime,
    MinuteOutOfRange,
    SecondOutOfRange,
    MicrosecondOutOfRange,
    MillisecondOutOfRange,
    NanosecondOutOfRange,
    MalformedTimeZoneOffset,
    TimezoneOffsetHourOutOfRange,
}

#[derive(Clone, Eq, PartialEq)]
pub struct DecodeError {
    pub reason: ErrorReason,
    pub debug_bytes: Vec<u8>,
}
impl Debug for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DecodeError: {:?}: '{}'",
            self.reason,
            escape_ascii(&self.debug_bytes)
        )
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

enum Date {
    Year { y: u16 },
    YearMonth { y: u16, mo: u8 },
    YearMonthDay { y: u16, mo: u8, d: u8 },
    YearWeek { y: u16, w: u8 },
    YearWeekDay { y: u16, w: u8, d: u8 },
}

enum MonthOrWeek {
    Month(u8),
    Week(u8),
}

enum Time {
    Hour { h: u8 },
    HourMinute { h: u8, m: u8 },
    HourMinuteSecond { h: u8, m: u8, s: u8 },
    HourMinuteMillisecond { h: u8, m: u8, ms: u16 },
    HourMinuteMicrosecond { h: u8, m: u8, us: u32 },
    HourMinuteNanosecond { h: u8, m: u8, ns: u32 },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct TzOffset {
    pub h: i8,
    pub m: u8, // Only one country has used a -00xx timezone, Liberia.  They stopped in 1972.
}

enum DateTimeTzOffset {
    Date(Date),
    DateTime(Date, Time),
    DateTz(Date, TzOffset),
    DateTimeTz(Date, Time, TzOffset),
    Time(Time),
    TimeTz(Time, TzOffset),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Year {
    pub y: u16,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearTzOffset {
    pub y: u16,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonth {
    pub y: u16,
    pub mo: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeek {
    pub y: u16,
    pub w: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthTzOffset {
    pub y: u16,
    pub mo: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekTzOffset {
    pub y: u16,
    pub w: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDay {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDay {
    pub y: u16,
    pub w: u8,
    pub d: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHour {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHour {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinute {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinute {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteSecond {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub s: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteSecond {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub s: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteSecondTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub s: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteSecondTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub s: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteMillisecond {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ms: u16,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteMillisecond {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ms: u16,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteMillisecondTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ms: u16,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteMillisecondTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ms: u16,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteMicrosecond {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub us: u32,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteMicrosecond {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub us: u32,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteMicrosecondTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub us: u32,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteMicrosecondTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub us: u32,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteNanosecond {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u32,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteNanosecond {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u32,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteNanosecondTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u32,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteNanosecondTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u32,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteSecond {
    pub h: u8,
    pub m: u8,
    pub s: u8,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteSecondTzOffset {
    pub h: u8,
    pub m: u8,
    pub s: u8,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteMillisecond {
    pub h: u8,
    pub m: u8,
    pub ms: u16,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteMillisecondTzOffset {
    pub h: u8,
    pub m: u8,
    pub ms: u16,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteMicrosecond {
    pub h: u8,
    pub m: u8,
    pub us: u32,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteMicrosecondTzOffset {
    pub h: u8,
    pub m: u8,
    pub us: u32,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteNanosecond {
    pub h: u8,
    pub m: u8,
    pub ns: u32,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteNanosecondTzOffset {
    pub h: u8,
    pub m: u8,
    pub ns: u32,
    pub tz: TzOffset,
}

#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    debug_bytes: &'a [u8],
    consumed_list_element: bool,
    list_depth: usize,
}
impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            debug_bytes: bytes,
            consumed_list_element: false,
            list_depth: 0,
        }
    }

    pub fn close(self) -> Result<(), DecodeError> {
        if self.list_depth != 0 {
            return Err(self.err(ErrorReason::ListEndNotConsumed));
        }
        if !self.bytes.is_empty() {
            return Err(self.err(ErrorReason::DataNotConsumed));
        }
        Ok(())
    }

    fn err(&self, reason: ErrorReason) -> DecodeError {
        let debug_bytes = Vec::from_iter(self.debug_bytes.iter().take(30).copied());
        DecodeError {
            reason,
            debug_bytes,
        }
    }

    pub fn consume_bool(&mut self) -> Result<bool, DecodeError> {
        let value = match self.consume_byte() {
            Some(b'T') => true,
            Some(b'F') => false,
            _ => return Err(self.err(ErrorReason::ExpectedBool)),
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
                        return Err(self.err(ErrorReason::ExpectedInteger));
                    }
                }
                b'0'..=b'9' => {
                    group_digit_count += 1;
                    let d = i64::from(b - b'0');
                    value = value
                        .checked_mul(10)
                        .ok_or_else(|| self.err(ErrorReason::IntegerTooLarge))?;
                    value = value
                        .checked_add(sign * d)
                        .ok_or_else(|| self.err(ErrorReason::IntegerTooLarge))?;
                    if value == 0 && 1 < group_digit_count {
                        return Err(self.err(ErrorReason::ExpectedSingleZero));
                    }
                }
                b'_' => {
                    if seen_underscore {
                        if group_digit_count != 3 {
                            return Err(self.err(ErrorReason::IncorrectDigitGrouping));
                        }
                    } else {
                        if value == 0 {
                            return Err(self.err(ErrorReason::IncorrectDigitGrouping));
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
            return Err(self.err(ErrorReason::ExpectedInteger));
        }
        if seen_underscore && group_digit_count != 3 {
            return Err(self.err(ErrorReason::IncorrectDigitGrouping));
        }
        if !seen_underscore && 3 < group_digit_count {
            return Err(self.err(ErrorReason::IncorrectDigitGrouping));
        }
        if sign == -1 && value == 0 {
            return Err(self.err(ErrorReason::NegativeZero));
        }
        self.close_item()?;
        Ok(value)
    }

    pub fn consume_open_list(&mut self) -> Result<(), DecodeError> {
        self.consume_exact(b'[')
            .ok_or_else(|| self.err(ErrorReason::ExpectedString))?;
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
                _ => Err(self.err(ErrorReason::ExpectedListSeparator)),
            }
        };
        self.debug_bytes = self.bytes;
        result
    }

    pub fn consume_date_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if (b'0'..=b'9').contains(&b) => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedDateTimeTzOffset)),
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
            return Err(self.err(ErrorReason::NotInList));
        }
        self.consume_exact(b']')
            .ok_or_else(|| self.err(ErrorReason::ExpectedListEnd))?;
        self.consumed_list_element = false;
        self.close_item()?;
        self.list_depth -= 1;
        Ok(())
    }

    pub fn consume_string(&mut self) -> Result<String, DecodeError> {
        match self.bytes.first() {
            Some(b'"') => {}
            _ => return Err(self.err(ErrorReason::ExpectedString)),
        }
        let Some((len, _)) = self
            .bytes
            .iter()
            .copied()
            .enumerate()
            .skip(1)
            .find(|(_n, b)| b == &b'"')
        else {
            return Err(self.err(ErrorReason::UnclosedString));
        };
        let s = core::str::from_utf8(&self.bytes[1..len])
            .map_err(|_e| self.err(ErrorReason::NotUtf8))?;
        let mut value = String::with_capacity(len - 1);
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                let opt_c1 = chars.next();
                let opt_c2 = chars.next();
                let (Some(c1), Some(c2)) = (opt_c1, opt_c2) else {
                    return Err(DecodeError {
                        reason: ErrorReason::IncompleteEscapeSequence,
                        debug_bytes: [Some('\\'), opt_c1, opt_c2, Some('"')]
                            .iter()
                            .flatten()
                            .collect::<String>()
                            .into_bytes(),
                    });
                };
                let (Some(b1), Some(b2)) = (c1.to_digit(16), c2.to_digit(16)) else {
                    return Err(DecodeError {
                        reason: ErrorReason::InvalidEscapeSequence,
                        debug_bytes: format!("\\{c1}{c2}").into_bytes(),
                    });
                };
                let b = 16 * b1 + b2;
                match b {
                    0x00..=0x1f | 0x22 | 0x5c | 0x7f => value.push(char::from_u32(b).unwrap()),
                    _ => {
                        return Err(DecodeError {
                            reason: ErrorReason::InvalidEscapeSequence,
                            debug_bytes: format!("\\{c1}{c2}").into_bytes(),
                        });
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

    fn consume_date(&mut self) -> Result<Date, DecodeError> {
        if self.consume_byte() != Some(b'D') {
            return Err(self.err(ErrorReason::ExpectedDate));
        }
        let d0 = u16::from(self.consume_date_digit()?);
        let d1 = u16::from(self.consume_date_digit()?);
        let d2 = u16::from(self.consume_date_digit()?);
        let d3 = u16::from(self.consume_date_digit()?);
        let y = 1000 * d0 + 100 * d1 + 10 * d2 + d3;
        if !(1..=9999).contains(&y) {
            return Err(self.err(ErrorReason::YearOutOfRange));
        }
        match self.bytes.first() {
            Some(b'-') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return Ok(Date::Year { y });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedDateTimeTzOffset)),
        };
        let month_or_week = if self.consume_exact(b'W').is_some() {
            let d0 = u8::from(self.consume_date_digit()?);
            let d1 = u8::from(self.consume_date_digit()?);
            let w = 10 * d0 + d1;
            if !(1..=53).contains(&w) {
                return Err(self.err(ErrorReason::WeekOutOfRange));
            }
            MonthOrWeek::Week(w)
        } else {
            let d0 = u8::from(self.consume_date_digit()?);
            let d1 = u8::from(self.consume_date_digit()?);
            let mo = 10 * d0 + d1;
            if !(1..=12).contains(&mo) {
                return Err(self.err(ErrorReason::MonthOutOfRange));
            }
            MonthOrWeek::Month(mo)
        };
        match self.bytes.first() {
            Some(b'-') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return match month_or_week {
                    MonthOrWeek::Month(mo) => Ok(Date::YearMonth { y, mo }),
                    MonthOrWeek::Week(w) => Ok(Date::YearWeek { y, w }),
                }
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedDateTimeTzOffset)),
        };
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let d = 10 * d0 + d1;
        if !(1..=31).contains(&d) {
            return Err(self.err(ErrorReason::DayOutOfRange));
        }
        match month_or_week {
            MonthOrWeek::Month(mo) => Ok(Date::YearMonthDay { y, mo, d }),
            MonthOrWeek::Week(w) => Ok(Date::YearWeekDay { y, w, d }),
        }
    }

    fn consume_time(&mut self) -> Result<Time, DecodeError> {
        if self.consume_byte() != Some(b'T') {
            return Err(self.err(ErrorReason::ExpectedTime));
        }
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let h = 10 * d0 + d1;
        if !(0..=23).contains(&h) {
            return Err(self.err(ErrorReason::HourOutOfRange));
        }
        match self.bytes.first() {
            Some(b':') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return Ok(Time::Hour { h });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedTime)),
        };
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let m = 10 * d0 + d1;
        if !(0..=59).contains(&m) {
            return Err(self.err(ErrorReason::MinuteOutOfRange));
        }
        match self.bytes.first() {
            Some(b':') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return Ok(Time::HourMinute { h, m });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedTime)),
        };
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let s = 10 * d0 + d1;
        if !(0..=60).contains(&s) {
            return Err(self.err(ErrorReason::SecondOutOfRange));
        }
        match self.bytes.first() {
            Some(b'_') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return Ok(Time::HourMinuteSecond { h, m, s });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedTime)),
        };
        let d0 = u16::from(self.consume_date_digit()?);
        let d1 = u16::from(self.consume_date_digit()?);
        let d2 = u16::from(self.consume_date_digit()?);
        let ms = 1000 * u16::from(s) + 100 * d0 + 10 * d1 + d2;
        match self.bytes.first() {
            Some(b'_') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return Ok(Time::HourMinuteMillisecond { h, m, ms });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedTime)),
        };
        let d0 = u32::from(self.consume_date_digit()?);
        let d1 = u32::from(self.consume_date_digit()?);
        let d2 = u32::from(self.consume_date_digit()?);
        let us = 1000 * u32::from(ms) + 100 * d0 + 10 * d1 + d2;
        match self.bytes.first() {
            Some(b'_') => self.consume_byte(),
            None | Some(b'Z') | Some(b'+') | Some(b'~') | Some(b',') | Some(b']') => {
                return Ok(Time::HourMinuteMicrosecond { h, m, us });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedTime)),
        };
        let d0 = u32::from(self.consume_date_digit()?);
        let d1 = u32::from(self.consume_date_digit()?);
        let d2 = u32::from(self.consume_date_digit()?);
        let ns = 1000 * us + 100 * d0 + 10 * d1 + d2;
        Ok(Time::HourMinuteNanosecond { h, m, ns })
    }

    fn consume_tz_offset(&mut self) -> Result<TzOffset, DecodeError> {
        let sign = match self.bytes.first() {
            Some(b'Z') => {
                self.consume_byte();
                return Ok(TzOffset { h: 0, m: 0 });
            }
            Some(b'+') => 1,
            Some(b'~') => -1,
            _ => return Err(self.err(ErrorReason::MalformedTimeZoneOffset)),
        };
        self.consume_byte();
        let d0 = i8::try_from(self.consume_date_digit()?).unwrap();
        let d1 = i8::try_from(self.consume_date_digit()?).unwrap();
        let h = sign * (10 * d0 + d1);
        if !(-23..=23).contains(&h) {
            return Err(self.err(ErrorReason::TimezoneOffsetHourOutOfRange));
        }
        match self.bytes.first() {
            Some(b':') => self.consume_byte(),
            None | Some(b',') | Some(b']') => {
                return Ok(TzOffset { h, m: 0 });
            }
            Some(..) => return Err(self.err(ErrorReason::MalformedTime)),
        };
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        let m = 10 * d0 + d1;
        if !(0..=59).contains(&m) {
            return Err(self.err(ErrorReason::MinuteOutOfRange));
        }
        Ok(TzOffset { h, m })
    }

    fn consume_date_time_tz_offset(&mut self) -> Result<DateTimeTzOffset, DecodeError> {
        let opt_date = if self.bytes.first().copied() == Some(b'D') {
            Some(self.consume_date()?)
        } else {
            None
        };
        let opt_time = if self.bytes.first().copied() == Some(b'T') {
            Some(self.consume_time()?)
        } else {
            None
        };
        if opt_date.is_none() && opt_time.is_none() {
            return Err(self.err(ErrorReason::ExpectedDateOrTime));
        }
        match &opt_date {
            None => {}
            Some(Date::Year { .. } | Date::YearMonth { .. } | Date::YearWeek { .. }) => {
                if opt_time.is_some() {
                    return Err(self.err(ErrorReason::MalformedDateTimeTzOffset));
                }
            }
            Some(Date::YearMonthDay { .. } | Date::YearWeekDay { .. }) => {}
        }
        let opt_tz_offset = match self.bytes.first() {
            None | Some(&b',') | Some(&b']') => None,
            _ => Some(self.consume_tz_offset()?),
        };
        match (opt_date, opt_time, opt_tz_offset) {
            (Some(date), None, None) => Ok(DateTimeTzOffset::Date(date)),
            (Some(date), Some(time), None) => Ok(DateTimeTzOffset::DateTime(date, time)),
            (Some(date), None, Some(tz)) => Ok(DateTimeTzOffset::DateTz(date, tz)),
            (Some(date), Some(time), Some(tz)) => Ok(DateTimeTzOffset::DateTimeTz(date, time, tz)),
            (None, Some(time), None) => Ok(DateTimeTzOffset::Time(time)),
            (None, Some(time), Some(tz)) => Ok(DateTimeTzOffset::TimeTz(time, tz)),
            (None, None, _) => unreachable!(),
        }
    }

    pub fn consume_year(&mut self) -> Result<Year, DecodeError> {
        if let DateTimeTzOffset::Date(Date::Year { y }) = self.consume_date_time_tz_offset()? {
            self.close_item()?;
            Ok(Year { y })
        } else {
            Err(self.err(ErrorReason::ExpectedYear))
        }
    }

    pub fn consume_year_tz_offset(&mut self) -> Result<YearTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTz(Date::Year { y }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearTzOffset { y, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearTzOffset))
        }
    }

    pub fn consume_year_month(&mut self) -> Result<YearMonth, DecodeError> {
        if let DateTimeTzOffset::Date(Date::YearMonth { y, mo }) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonth { y, mo })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonth))
        }
    }

    pub fn consume_year_week(&mut self) -> Result<YearWeek, DecodeError> {
        if let DateTimeTzOffset::Date(Date::YearWeek { y, w }) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeek { y, w })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeek))
        }
    }

    pub fn consume_year_month_tz_offset(&mut self) -> Result<YearMonthTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTz(Date::YearMonth { y, mo }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthTzOffset { y, mo, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthTzOffset))
        }
    }

    pub fn consume_year_week_tz_offset(&mut self) -> Result<YearWeekTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTz(Date::YearWeek { y, w }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekTzOffset { y, w, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekTzOffset))
        }
    }

    pub fn consume_year_month_day(&mut self) -> Result<YearMonthDay, DecodeError> {
        if let DateTimeTzOffset::Date(Date::YearMonthDay { y, mo, d }) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDay { y, mo, d })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDay))
        }
    }

    pub fn consume_year_week_day(&mut self) -> Result<YearWeekDay, DecodeError> {
        if let DateTimeTzOffset::Date(Date::YearWeekDay { y, w, d }) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDay { y, w, d })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDay))
        }
    }

    pub fn consume_year_month_day_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTz(Date::YearMonthDay { y, mo, d }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayTzOffset { y, mo, d, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayTzOffset))
        }
    }

    pub fn consume_year_week_day_tz_offset(&mut self) -> Result<YearWeekDayTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTz(Date::YearWeekDay { y, w, d }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayTzOffset { y, w, d, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayTzOffset))
        }
    }

    pub fn consume_year_month_day_hour(&mut self) -> Result<YearMonthDayHour, DecodeError> {
        if let DateTimeTzOffset::DateTime(Date::YearMonthDay { y, mo, d }, Time::Hour { h }) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHour { y, mo, d, h })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHour))
        }
    }

    pub fn consume_year_week_day_hour(&mut self) -> Result<YearWeekDayHour, DecodeError> {
        if let DateTimeTzOffset::DateTime(Date::YearWeekDay { y, w, d }, Time::Hour { h }) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHour { y, w, d, h })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHour))
        }
    }

    pub fn consume_year_month_day_hour_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(Date::YearMonthDay { y, mo, d }, Time::Hour { h }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourTzOffset { y, mo, d, h, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourTzOffset))
        }
    }

    pub fn consume_year_week_day_hour_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(Date::YearWeekDay { y, w, d }, Time::Hour { h }, tz) =
            self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourTzOffset { y, w, d, h, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourTzOffset))
        }
    }

    pub fn consume_year_month_day_hour_minute(
        &mut self,
    ) -> Result<YearMonthDayHourMinute, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinute { h, m },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinute { y, mo, d, h, m })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinute))
        }
    }

    pub fn consume_year_week_day_hour_minute(
        &mut self,
    ) -> Result<YearWeekDayHourMinute, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearWeekDay { y, w, d },
            Time::HourMinute { h, m },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinute { y, w, d, h, m })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinute))
        }
    }

    pub fn consume_year_month_day_hour_minute_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinute { h, m },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteTzOffset { y, mo, d, h, m, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteTzOffset))
        }
    }

    pub fn consume_year_week_day_hour_minute_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearWeekDay { y, w, d },
            Time::HourMinute { h, m },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteTzOffset { y, w, d, h, m, tz })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteTzOffset))
        }
    }

    pub fn consume_year_month_day_hour_minute_second(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteSecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteSecond { h, m, s },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteSecond { y, mo, d, h, m, s })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteSecond))
        }
    }

    pub fn consume_year_week_day_hour_minute_second(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteSecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteSecond { h, m, s },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteSecond { y, w, d, h, m, s })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteSecond))
        }
    }

    pub fn consume_year_month_day_hour_minute_second_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteSecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteSecond { h, m, s },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteSecondTzOffset {
                y,
                mo,
                d,
                h,
                m,
                s,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteSecondTzOffset))
        }
    }

    pub fn consume_year_week_day_hour_minute_second_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteSecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteSecond { h, m, s },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteSecondTzOffset {
                y,
                w,
                d,
                h,
                m,
                s,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteSecondTzOffset))
        }
    }

    pub fn consume_year_month_day_hour_minute_millisecond(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMillisecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteMillisecond { h, m, ms },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteMillisecond { y, mo, d, h, m, ms })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteMillisecond))
        }
    }

    pub fn consume_year_week_day_hour_minute_millisecond(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMillisecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteMillisecond { h, m, ms },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteMillisecond { y, w, d, h, m, ms })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteMillisecond))
        }
    }

    pub fn consume_year_month_day_hour_minute_millisecond_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMillisecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteMillisecond { h, m, ms },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteMillisecondTzOffset {
                y,
                mo,
                d,
                h,
                m,
                ms,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteMillisecondTzOffset))
        }
    }

    pub fn consume_year_week_day_hour_minute_millisecond_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMillisecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteMillisecond { h, m, ms },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteMillisecondTzOffset {
                y,
                w,
                d,
                h,
                m,
                ms,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteMillisecondTzOffset))
        }
    }

    pub fn consume_year_month_day_hour_minute_microsecond(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMicrosecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteMicrosecond { h, m, us },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteMicrosecond { y, mo, d, h, m, us })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecond))
        }
    }

    pub fn consume_year_week_day_hour_minute_microsecond(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMicrosecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteMicrosecond { h, m, us },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteMicrosecond { y, w, d, h, m, us })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteMicrosecond))
        }
    }

    pub fn consume_year_month_day_hour_minute_microsecond_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMicrosecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteMicrosecond { h, m, us },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteMicrosecondTzOffset {
                y,
                mo,
                d,
                h,
                m,
                us,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecondTzOffset))
        }
    }

    pub fn consume_year_week_day_hour_minute_microsecond_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMicrosecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteMicrosecond { h, m, us },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteMicrosecondTzOffset {
                y,
                w,
                d,
                h,
                m,
                us,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteMicrosecondTzOffset))
        }
    }

    pub fn consume_year_month_day_hour_minute_nanosecond(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteNanosecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteNanosecond { h, m, ns },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteNanosecond { y, mo, d, h, m, ns })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteNanosecond))
        }
    }

    pub fn consume_year_week_day_hour_minute_nanosecond(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteNanosecond, DecodeError> {
        if let DateTimeTzOffset::DateTime(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteNanosecond { h, m, ns },
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteNanosecond { y, w, d, h, m, ns })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteNanosecond))
        }
    }

    pub fn consume_year_month_day_hour_minute_nanosecond_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteNanosecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearMonthDay { y, mo, d },
            Time::HourMinuteNanosecond { h, m, ns },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearMonthDayHourMinuteNanosecondTzOffset {
                y,
                mo,
                d,
                h,
                m,
                ns,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearMonthDayHourMinuteNanosecondTzOffset))
        }
    }

    pub fn consume_year_week_day_hour_minute_nanosecond_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteNanosecondTzOffset, DecodeError> {
        if let DateTimeTzOffset::DateTimeTz(
            Date::YearWeekDay { y, w, d },
            Time::HourMinuteNanosecond { h, m, ns },
            tz,
        ) = self.consume_date_time_tz_offset()?
        {
            self.close_item()?;
            Ok(YearWeekDayHourMinuteNanosecondTzOffset {
                y,
                w,
                d,
                h,
                m,
                ns,
                tz,
            })
        } else {
            Err(self.err(ErrorReason::ExpectedYearWeekDayHourMinuteNanosecondTzOffset))
        }
    }
}
