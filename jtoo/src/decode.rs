use crate::escape_ascii;
use core::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorReason {
    DataNotConsumed,
    DayOutOfRange,
    ExpectedBool,
    ExpectedDate,
    ExpectedDateOrTime,
    ExpectedDay,
    ExpectedHour,
    ExpectedInteger,
    ExpectedList,
    ExpectedListEnd,
    ExpectedListSeparator,
    ExpectedMicrosecond,
    ExpectedMillisecond,
    ExpectedMinute,
    ExpectedMonth,
    ExpectedNanosecond,
    ExpectedSecond,
    ExpectedSingleZero,
    ExpectedString,
    ExpectedTime,
    ExpectedTzOffset,
    ExpectedWeek,
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
    HourOutOfRange,
    IncompleteEscapeSequence,
    IncorrectDigitGrouping,
    IntegerTooLarge,
    InvalidEscapeSequence,
    ListEndNotConsumed,
    MalformedBool,
    MalformedDate,
    MalformedDateTimeTzOffset,
    MalformedInteger,
    MalformedListEnd,
    MalformedMonth,
    MalformedString,
    MalformedTime,
    MalformedTimeZoneOffset,
    MalformedWeek,
    MalformedYear,
    MicrosecondOutOfRange,
    MillisecondOutOfRange,
    MinuteOutOfRange,
    MonthOutOfRange,
    NanosecondOutOfRange,
    NegativeZero,
    NotInList,
    NotUtf8,
    SecondOutOfRange,
    TimezoneOffsetHourOutOfRange,
    TimezoneOffsetMinuteOutOfRange,
    UnclosedString,
    UnconsumedData,
    Unimplemented,
    WeekOutOfRange,
    YearOutOfRange,
    ZeroTimeZoneMinutesShouldBeOmitted,
    ZeroTimeZoneOffsetShouldBeZ,
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

pub enum Date {
    Year { y: u16 },
    YearMonth { y: u16, mo: u8 },
    YearMonthDay { y: u16, mo: u8, d: u8 },
    YearWeek { y: u16, w: u8 },
    YearWeekDay { y: u16, w: u8, d: u8 },
}

pub enum Time {
    Hour { h: u8 },
    HourMinute { h: u8, m: u8 },
    HourMinuteSecond { h: u8, m: u8, s: u8 },
    HourMinuteMillisecond { h: u8, m: u8, ms: u16 },
    HourMinuteMicrosecond { h: u8, m: u8, us: u32 },
    HourMinuteNanosecond { h: u8, m: u8, ns: u64 },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct TzOffset {
    pub h: i8,
    pub m: u8, // Only one country has used a -00xx timezone, Liberia.  They stopped in 1972.
}

pub enum DateTimeTzOffset {
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
    pub ns: u64,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteNanosecond {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u64,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearMonthDayHourMinuteNanosecondTzOffset {
    pub y: u16,
    pub mo: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u64,
    pub tz: TzOffset,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct YearWeekDayHourMinuteNanosecondTzOffset {
    pub y: u16,
    pub w: u8,
    pub d: u8,
    pub h: u8,
    pub m: u8,
    pub ns: u64,
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
    pub ns: u64,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct HourMinuteNanosecondTzOffset {
    pub h: u8,
    pub m: u8,
    pub ns: u64,
    pub tz: TzOffset,
}

#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    debug_bytes: &'a [u8],
    list_depth: usize,
}
impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            debug_bytes: bytes,
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
        self.close_item(ErrorReason::MalformedBool)?;
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
        self.close_item(ErrorReason::MalformedInteger)?;
        Ok(value)
    }

    pub fn consume_open_list(&mut self) -> Result<(), DecodeError> {
        self.consume_exact(b'[')
            .ok_or_else(|| self.err(ErrorReason::ExpectedList))?;
        self.list_depth += 1;
        Ok(())
    }

    pub fn close_item(&mut self, reason: ErrorReason) -> Result<(), DecodeError> {
        if self.list_depth == 0 {
            if !self.bytes.is_empty() {
                return Err(self.err(reason));
            }
        } else {
            match self.bytes.first() {
                Some(&b',') => {
                    self.consume_byte();
                }
                Some(&b']') => {}
                None => {} // Next call will try to consume list close and fail.
                _ => return Err(self.err(ErrorReason::ExpectedListSeparator)),
            }
        }
        self.debug_bytes = self.bytes;
        Ok(())
    }

    pub fn consume_date_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if (b'0'..=b'9').contains(&b) => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedDate)),
        }
    }

    pub fn consume_tz_offset_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if (b'0'..=b'9').contains(&b) => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedTimeZoneOffset)),
        }
    }

    pub fn consume_time_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if (b'0'..=b'9').contains(&b) => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedTime)),
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
        self.close_item(ErrorReason::MalformedListEnd)?;
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
        self.close_item(ErrorReason::MalformedString)?;
        Ok(value)
    }

    pub fn consume_year_part(&mut self) -> Result<u16, DecodeError> {
        if self.consume_byte() != Some(b'D') {
            return Err(self.err(ErrorReason::ExpectedYear));
        }
        let d0 = u16::from(self.consume_date_digit()?);
        let d1 = u16::from(self.consume_date_digit()?);
        let d2 = u16::from(self.consume_date_digit()?);
        let d3 = u16::from(self.consume_date_digit()?);
        match self.bytes.first().copied() {
            Some(b'-' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        let y = 1000 * d0 + 100 * d1 + 10 * d2 + d3;
        if !(1..=9999).contains(&y) {
            return Err(self.err(ErrorReason::YearOutOfRange));
        }
        Ok(y)
    }

    pub fn consume_month(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedMonth))?;
        match self.bytes.first().copied() {
            None => return Err(self.err(ErrorReason::MalformedDate)),
            Some(b'W') => return Err(self.err(ErrorReason::ExpectedMonth)),
            _ => {}
        }
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        match self.bytes.first().copied() {
            Some(b'-' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        let mo = 10 * d0 + d1;
        if !(1..=12).contains(&mo) {
            return Err(self.err(ErrorReason::MonthOutOfRange));
        }
        Ok(mo)
    }

    pub fn consume_week(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedWeek))?;
        match self.consume_byte() {
            Some(b'W') => {}
            Some(b'0'..=b'9') => return Err(self.err(ErrorReason::ExpectedWeek)),
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        match self.bytes.first().copied() {
            Some(b'-' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        let w = 10 * d0 + d1;
        if !(1..=53).contains(&w) {
            return Err(self.err(ErrorReason::WeekOutOfRange));
        }
        Ok(w)
    }

    pub fn consume_day(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedDay))?;
        let d0 = u8::from(self.consume_date_digit()?);
        let d1 = u8::from(self.consume_date_digit()?);
        match self.bytes.first().copied() {
            Some(b'T' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        let d = 10 * d0 + d1;
        if !(1..=31).contains(&d) {
            return Err(self.err(ErrorReason::DayOutOfRange));
        }
        Ok(d)
    }

    fn consume_date(&mut self) -> Result<Date, DecodeError> {
        let y = self.consume_year_part()?;
        if self.bytes.first().copied() != Some(b'-') {
            return Ok(Date::Year { y });
        }
        enum MonthOrWeek {
            Month(u8),
            Week(u8),
        }
        let month_or_week = if self.bytes.get(1).copied() == Some(b'W') {
            MonthOrWeek::Week(self.consume_week()?)
        } else {
            MonthOrWeek::Month(self.consume_month()?)
        };
        if self.bytes.first().copied() != Some(b'-') {
            return match month_or_week {
                MonthOrWeek::Month(mo) => Ok(Date::YearMonth { y, mo }),
                MonthOrWeek::Week(w) => Ok(Date::YearWeek { y, w }),
            };
        }
        let d = self.consume_day()?;
        match month_or_week {
            MonthOrWeek::Month(mo) => Ok(Date::YearMonthDay { y, mo, d }),
            MonthOrWeek::Week(w) => Ok(Date::YearWeekDay { y, w, d }),
        }
    }

    fn consume_hour(&mut self) -> Result<u8, DecodeError> {
        if self.consume_byte() != Some(b'T') {
            return Err(self.err(ErrorReason::ExpectedHour));
        }
        let d0 = u8::from(self.consume_time_digit()?);
        let d1 = u8::from(self.consume_time_digit()?);
        match self.bytes.first().copied() {
            Some(b':' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTime)),
        }
        let h = 10 * d0 + d1;
        if !(0..=23).contains(&h) {
            return Err(self.err(ErrorReason::HourOutOfRange));
        }
        Ok(h)
    }

    fn consume_minute(&mut self) -> Result<u8, DecodeError> {
        if self.consume_byte() != Some(b':') {
            return Err(self.err(ErrorReason::ExpectedMinute));
        }
        let d0 = u8::from(self.consume_time_digit()?);
        let d1 = u8::from(self.consume_time_digit()?);
        match self.bytes.first().copied() {
            Some(b':' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTime)),
        }
        let m = 10 * d0 + d1;
        if !(0..=59).contains(&m) {
            return Err(self.err(ErrorReason::MinuteOutOfRange));
        }
        Ok(m)
    }

    fn consume_second(&mut self) -> Result<u8, DecodeError> {
        if self.consume_byte() != Some(b':') {
            return Err(self.err(ErrorReason::ExpectedSecond));
        }
        let d0 = u8::from(self.consume_time_digit()?);
        let d1 = u8::from(self.consume_time_digit()?);
        match self.bytes.first().copied() {
            Some(b'.' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTime)),
        }
        let s = 10 * d0 + d1;
        if !(0..=60).contains(&s) {
            return Err(self.err(ErrorReason::SecondOutOfRange));
        }
        Ok(s)
    }

    fn consume_millisecond_part(&mut self) -> Result<u16, DecodeError> {
        if self.consume_byte() != Some(b'.') {
            return Err(self.err(ErrorReason::ExpectedMillisecond));
        }
        let d0 = u16::from(self.consume_time_digit()?);
        let d1 = u16::from(self.consume_time_digit()?);
        let d2 = u16::from(self.consume_time_digit()?);
        match self.bytes.first().copied() {
            Some(b'_' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTime)),
        }
        let ms = 100 * d0 + 10 * d1 + d2;
        Ok(ms)
    }

    fn consume_millisecond(&mut self) -> Result<u16, DecodeError> {
        let s = self.consume_second()?;
        let ms = self.consume_millisecond_part()?;
        Ok(1000 * u16::from(s) + ms)
    }

    fn consume_microsecond_part(&mut self) -> Result<u32, DecodeError> {
        if self.consume_byte() != Some(b'_') {
            return Err(self.err(ErrorReason::ExpectedMicrosecond));
        }
        let d0 = u16::from(self.consume_time_digit()?);
        let d1 = u16::from(self.consume_time_digit()?);
        let d2 = u16::from(self.consume_time_digit()?);
        match self.bytes.first().copied() {
            Some(b'_' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTime)),
        }
        let us = 100 * d0 + 10 * d1 + d2;
        Ok(u32::from(us))
    }

    fn consume_microsecond(&mut self) -> Result<u32, DecodeError> {
        let s = self.consume_second()?;
        let ms = self.consume_millisecond_part()?;
        let us = self.consume_microsecond_part()?;
        Ok(1_000_000 * u32::from(s) + 1_000 * u32::from(ms) + us)
    }

    fn consume_nanosecond_part(&mut self) -> Result<u64, DecodeError> {
        if self.consume_byte() != Some(b'_') {
            return Err(self.err(ErrorReason::ExpectedNanosecond));
        }
        let d0 = u64::from(self.consume_time_digit()?);
        let d1 = u64::from(self.consume_time_digit()?);
        let d2 = u64::from(self.consume_time_digit()?);
        match self.bytes.first().copied() {
            Some(b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTime)),
        }
        let ns = 100 * d0 + 10 * d1 + d2;
        Ok(ns)
    }

    fn consume_nanosecond(&mut self) -> Result<u64, DecodeError> {
        let s = self.consume_second()?;
        let ms = self.consume_millisecond_part()?;
        let us = self.consume_microsecond_part()?;
        let ns = self.consume_nanosecond_part()?;
        Ok(1_000_000_000 * u64::from(s) + 1_000_000 * u64::from(ms) + 1_000 * u64::from(us) + ns)
    }

    fn consume_time(&mut self) -> Result<Time, DecodeError> {
        let h = self.consume_hour()?;
        if self.bytes.first().copied() != Some(b':') {
            return Ok(Time::Hour { h });
        }
        let m = self.consume_minute()?;
        if self.bytes.first().copied() != Some(b':') {
            return Ok(Time::HourMinute { h, m });
        }
        let s = self.consume_second()?;
        if self.bytes.first().copied() != Some(b'_') {
            return Ok(Time::HourMinuteSecond { h, m, s });
        }
        let ms = 1000 * u16::from(s) + self.consume_millisecond_part()?;
        if self.bytes.first().copied() != Some(b'_') {
            return Ok(Time::HourMinuteMillisecond { h, m, ms });
        }
        let us = 1000 * u32::from(ms) + self.consume_microsecond_part()?;
        if self.bytes.first().copied() != Some(b'_') {
            return Ok(Time::HourMinuteMicrosecond { h, m, us });
        }
        let ns = 1000 * u64::from(ms) + self.consume_nanosecond_part()?;
        Ok(Time::HourMinuteNanosecond { h, m, ns })
    }

    fn consume_tz_offset(&mut self, reason: ErrorReason) -> Result<TzOffset, DecodeError> {
        let sign = match self.consume_byte() {
            Some(b'-' | b'T' | b':' | b'.' | b'_') => return Err(self.err(reason)),
            Some(b',' | b']') | None => return Err(self.err(ErrorReason::ExpectedTzOffset)),
            Some(b'Z') => {
                return Ok(TzOffset { h: 0, m: 0 });
            }
            Some(b'+') => 1,
            Some(b'~') => -1,
            _ => return Err(self.err(ErrorReason::MalformedTimeZoneOffset)),
        };
        let d0 = i8::try_from(self.consume_tz_offset_digit()?).unwrap();
        let d1 = i8::try_from(self.consume_tz_offset_digit()?).unwrap();
        let h = sign * (10 * d0 + d1);
        if !(-23..=23).contains(&h) {
            return Err(self.err(ErrorReason::TimezoneOffsetHourOutOfRange));
        }
        match self.bytes.first().copied() {
            Some(b':') => self.consume_byte(),
            Some(b',' | b']') | None => {
                if h == 0 {
                    return Err(self.err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ));
                }
                return Ok(TzOffset { h, m: 0 });
            }
            _ => return Err(self.err(ErrorReason::MalformedTimeZoneOffset)),
        };
        let d0 = u8::from(self.consume_tz_offset_digit()?);
        let d1 = u8::from(self.consume_tz_offset_digit()?);
        match self.bytes.first().copied() {
            Some(b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedTimeZoneOffset)),
        }
        let m = 10 * d0 + d1;
        if !(0..=59).contains(&m) {
            Err(self.err(ErrorReason::TimezoneOffsetMinuteOutOfRange))
        } else if h == 0 && m == 0 {
            Err(self.err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ))
        } else if m == 0 {
            Err(self.err(ErrorReason::ZeroTimeZoneMinutesShouldBeOmitted))
        } else {
            Ok(TzOffset { h, m })
        }
    }

    pub fn consume_date_time_tz_offset(&mut self) -> Result<DateTimeTzOffset, DecodeError> {
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
            _ => Some(self.consume_tz_offset(ErrorReason::ExpectedTzOffset)?),
        };
        self.close_item(ErrorReason::MalformedDateTimeTzOffset)?;
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
        let result = Ok(Year {
            y: self.consume_year_part()?,
        });
        self.close_item(ErrorReason::ExpectedYear)?;
        result
    }

    pub fn consume_year_tz_offset(&mut self) -> Result<YearTzOffset, DecodeError> {
        let result = Ok(YearTzOffset {
            y: self.consume_year_part()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearTzOffset)?;
        result
    }

    pub fn consume_year_month(&mut self) -> Result<YearMonth, DecodeError> {
        let result = Ok(YearMonth {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonth)?;
        result
    }

    pub fn consume_year_week(&mut self) -> Result<YearWeek, DecodeError> {
        let result = Ok(YearWeek {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeek)?;
        result
    }

    pub fn consume_year_month_tz_offset(&mut self) -> Result<YearMonthTzOffset, DecodeError> {
        let result = Ok(YearMonthTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearMonthTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthTzOffset)?;
        result
    }

    pub fn consume_year_week_tz_offset(&mut self) -> Result<YearWeekTzOffset, DecodeError> {
        let result = Ok(YearWeekTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearWeekTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekTzOffset)?;
        result
    }

    pub fn consume_year_month_day(&mut self) -> Result<YearMonthDay, DecodeError> {
        let result = Ok(YearMonthDay {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDay)?;
        result
    }

    pub fn consume_year_week_day(&mut self) -> Result<YearWeekDay, DecodeError> {
        let result = Ok(YearWeekDay {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDay)?;
        result
    }

    pub fn consume_year_month_day_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayTzOffset, DecodeError> {
        let result = Ok(YearMonthDayTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearMonthDayTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayTzOffset)?;
        result
    }

    pub fn consume_year_week_day_tz_offset(&mut self) -> Result<YearWeekDayTzOffset, DecodeError> {
        let result = Ok(YearWeekDayTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearWeekDayTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayTzOffset)?;
        result
    }

    pub fn consume_year_month_day_hour(&mut self) -> Result<YearMonthDayHour, DecodeError> {
        let result = Ok(YearMonthDayHour {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHour)?;
        result
    }

    pub fn consume_year_week_day_hour(&mut self) -> Result<YearWeekDayHour, DecodeError> {
        let result = Ok(YearWeekDayHour {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHour)?;
        result
    }

    pub fn consume_year_month_day_hour_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourTzOffset, DecodeError> {
        let result = Ok(YearMonthDayHourTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearMonthDayHourTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourTzOffset)?;
        result
    }

    pub fn consume_year_week_day_hour_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourTzOffset, DecodeError> {
        let result = Ok(YearWeekDayHourTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearWeekDayHourTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourTzOffset)?;
        result
    }

    pub fn consume_year_month_day_hour_minute(
        &mut self,
    ) -> Result<YearMonthDayHourMinute, DecodeError> {
        let result = Ok(YearMonthDayHourMinute {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinute)?;
        result
    }

    pub fn consume_year_week_day_hour_minute(
        &mut self,
    ) -> Result<YearWeekDayHourMinute, DecodeError> {
        let result = Ok(YearWeekDayHourMinute {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinute)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteTzOffset, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearMonthDayHourMinuteTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteTzOffset)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteTzOffset, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearWeekDayHourMinuteTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteTzOffset)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_second(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteSecond, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteSecond {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            s: self.consume_second()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteSecond)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_second(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteSecond, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteSecond {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            s: self.consume_second()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteSecond)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_second_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteSecondTzOffset, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteSecondTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            s: self.consume_second()?,
            tz: self
                .consume_tz_offset(ErrorReason::ExpectedYearMonthDayHourMinuteSecondTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteSecondTzOffset)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_second_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteSecondTzOffset, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteSecondTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            s: self.consume_second()?,
            tz: self.consume_tz_offset(ErrorReason::ExpectedYearWeekDayHourMinuteSecondTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteSecondTzOffset)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_millisecond(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMillisecond, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteMillisecond {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ms: self.consume_millisecond()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteMillisecond)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_millisecond(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMillisecond, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteMillisecond {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ms: self.consume_millisecond()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteMillisecond)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_millisecond_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMillisecondTzOffset, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteMillisecondTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ms: self.consume_millisecond()?,
            tz: self.consume_tz_offset(
                ErrorReason::ExpectedYearMonthDayHourMinuteMillisecondTzOffset,
            )?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteMillisecondTzOffset)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_millisecond_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMillisecondTzOffset, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteMillisecondTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ms: self.consume_millisecond()?,
            tz: self
                .consume_tz_offset(ErrorReason::ExpectedYearWeekDayHourMinuteMillisecondTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteMillisecondTzOffset)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_microsecond(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMicrosecond, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteMicrosecond {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            us: self.consume_microsecond()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecond)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_microsecond(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMicrosecond, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteMicrosecond {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            us: self.consume_microsecond()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteMicrosecond)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_microsecond_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteMicrosecondTzOffset, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteMicrosecondTzOffset {
            y: self.consume_year_part()?,
            m: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            mo: self.consume_minute()?,
            us: self.consume_microsecond()?,
            tz: self.consume_tz_offset(
                ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecondTzOffset,
            )?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecondTzOffset)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_microsecond_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteMicrosecondTzOffset, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteMicrosecondTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            us: self.consume_microsecond()?,
            tz: self
                .consume_tz_offset(ErrorReason::ExpectedYearWeekDayHourMinuteMicrosecondTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteMicrosecondTzOffset)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_nanosecond(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteNanosecond, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteNanosecond {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ns: self.consume_nanosecond()?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteNanosecond)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_nanosecond(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteNanosecond, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteNanosecond {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ns: self.consume_nanosecond()?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteNanosecond)?;
        result
    }

    pub fn consume_year_month_day_hour_minute_nanosecond_tz_offset(
        &mut self,
    ) -> Result<YearMonthDayHourMinuteNanosecondTzOffset, DecodeError> {
        let result = Ok(YearMonthDayHourMinuteNanosecondTzOffset {
            y: self.consume_year_part()?,
            mo: self.consume_month()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ns: self.consume_nanosecond()?,
            tz: self
                .consume_tz_offset(ErrorReason::ExpectedYearMonthDayHourMinuteNanosecondTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearMonthDayHourMinuteNanosecondTzOffset)?;
        result
    }

    pub fn consume_year_week_day_hour_minute_nanosecond_tz_offset(
        &mut self,
    ) -> Result<YearWeekDayHourMinuteNanosecondTzOffset, DecodeError> {
        let result = Ok(YearWeekDayHourMinuteNanosecondTzOffset {
            y: self.consume_year_part()?,
            w: self.consume_week()?,
            d: self.consume_day()?,
            h: self.consume_hour()?,
            m: self.consume_minute()?,
            ns: self.consume_nanosecond()?,
            tz: self
                .consume_tz_offset(ErrorReason::ExpectedYearWeekDayHourMinuteNanosecondTzOffset)?,
        });
        self.close_item(ErrorReason::ExpectedYearWeekDayHourMinuteNanosecondTzOffset)?;
        result
    }
}
