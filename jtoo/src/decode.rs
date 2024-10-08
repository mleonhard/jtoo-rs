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
    ListCloseNotConsumed,
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

#[allow(clippy::module_name_repetitions)]
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
    #[allow(clippy::missing_errors_doc)]
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError>
    where
        Self: Sized;

    #[allow(clippy::missing_errors_doc)]
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut decoder = Decoder::new(bytes);
        Self::decode_using(&mut decoder)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Date {
    Year { y: u16 },
    YearMonth { y: u16, mo: u8 },
    YearMonthDay { y: u16, mo: u8, d: u8 },
    YearWeek { y: u16, w: u8 },
    YearWeekDay { y: u16, w: u8, d: u8 },
}
impl Date {
    #[must_use]
    pub fn year(&self) -> Option<u16> {
        match self {
            Date::Year { y, .. }
            | Date::YearMonth { y, .. }
            | Date::YearMonthDay { y, .. }
            | Date::YearWeek { y, .. }
            | Date::YearWeekDay { y, .. } => Some(*y),
        }
    }

    #[must_use]
    pub fn month(&self) -> Option<u8> {
        match self {
            Date::Year { .. } | Date::YearWeek { .. } | Date::YearWeekDay { .. } => None,
            Date::YearMonth { mo, .. } | Date::YearMonthDay { mo, .. } => Some(*mo),
        }
    }

    #[must_use]
    pub fn week(&self) -> Option<u8> {
        match self {
            Date::Year { .. } | Date::YearMonth { .. } | Date::YearMonthDay { .. } => None,
            Date::YearWeek { w, .. } | Date::YearWeekDay { w, .. } => Some(*w),
        }
    }

    #[must_use]
    pub fn day(&self) -> Option<u8> {
        match self {
            Date::Year { .. } | Date::YearMonth { .. } | Date::YearWeek { .. } => None,
            Date::YearMonthDay { d, .. } | Date::YearWeekDay { d, .. } => Some(*d),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Time {
    Hour { h: u8 },
    HourMinute { h: u8, m: u8 },
    HourMinuteSecond { h: u8, m: u8, s: u8 },
    HourMinuteMillisecond { h: u8, m: u8, ms: u16 },
    HourMinuteMicrosecond { h: u8, m: u8, us: u32 },
    HourMinuteNanosecond { h: u8, m: u8, ns: u64 },
}
impl Time {
    #[must_use]
    pub fn hour(&self) -> Option<u8> {
        match self {
            Time::Hour { h, .. }
            | Time::HourMinute { h, .. }
            | Time::HourMinuteSecond { h, .. }
            | Time::HourMinuteMillisecond { h, .. }
            | Time::HourMinuteMicrosecond { h, .. }
            | Time::HourMinuteNanosecond { h, .. } => Some(*h),
        }
    }

    #[must_use]
    pub fn minute(&self) -> Option<u8> {
        match self {
            Time::Hour { .. } => None,
            Time::HourMinute { m, .. }
            | Time::HourMinuteSecond { m, .. }
            | Time::HourMinuteMillisecond { m, .. }
            | Time::HourMinuteMicrosecond { m, .. }
            | Time::HourMinuteNanosecond { m, .. } => Some(*m),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn second(&self) -> Option<u8> {
        match self {
            Time::Hour { .. } | Time::HourMinute { .. } => None,
            Time::HourMinuteSecond { s, .. } => Some(*s),
            Time::HourMinuteMillisecond { ms, .. } => Some(u8::try_from(*ms / 1_000).unwrap()),
            Time::HourMinuteMicrosecond { us, .. } => Some(u8::try_from(*us / 1_000_000).unwrap()),
            Time::HourMinuteNanosecond { ns, .. } => {
                Some(u8::try_from(*ns / 1_000_000_000).unwrap())
            }
        }
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn millisecond(&self) -> Option<u16> {
        match self {
            Time::Hour { .. } | Time::HourMinute { .. } | Time::HourMinuteSecond { .. } => None,
            Time::HourMinuteMillisecond { ms, .. } => Some(*ms),
            Time::HourMinuteMicrosecond { us, .. } => Some(u16::try_from(*us / 1_000).unwrap()),
            Time::HourMinuteNanosecond { ns, .. } => Some(u16::try_from(*ns / 1_000_000).unwrap()),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn microsecond(&self) -> Option<u32> {
        match self {
            Time::Hour { .. }
            | Time::HourMinute { .. }
            | Time::HourMinuteSecond { .. }
            | Time::HourMinuteMillisecond { .. } => None,
            Time::HourMinuteMicrosecond { us, .. } => Some(*us),
            Time::HourMinuteNanosecond { ns, .. } => Some(u32::try_from(*ns / 1_000).unwrap()),
        }
    }

    #[must_use]
    pub fn nanosecond(&self) -> Option<u64> {
        match self {
            Time::Hour { .. }
            | Time::HourMinute { .. }
            | Time::HourMinuteSecond { .. }
            | Time::HourMinuteMillisecond { .. }
            | Time::HourMinuteMicrosecond { .. } => None,
            Time::HourMinuteNanosecond { ns, .. } => Some(*ns),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct TzOffset {
    pub h: i8,
    pub m: u8, // Only one country has used a -00xx timezone, Liberia.  They stopped in 1972.
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum DateTimeTzOffset {
    Date(Date),
    DateTime(Date, Time),
    DateTz(Date, TzOffset),
    DateTimeTz(Date, Time, TzOffset),
    Time(Time),
    TimeTz(Time, TzOffset),
}
impl DateTimeTzOffset {
    #[must_use]
    pub fn date(&self) -> Option<&Date> {
        match self {
            DateTimeTzOffset::Date(d)
            | DateTimeTzOffset::DateTime(d, _)
            | DateTimeTzOffset::DateTz(d, _)
            | DateTimeTzOffset::DateTimeTz(d, _, _) => Some(d),
            DateTimeTzOffset::Time(..) | DateTimeTzOffset::TimeTz(..) => None,
        }
    }

    #[must_use]
    pub fn time(&self) -> Option<&Time> {
        match self {
            DateTimeTzOffset::Date(..) | DateTimeTzOffset::DateTz(..) => None,
            DateTimeTzOffset::DateTime(_, t)
            | DateTimeTzOffset::DateTimeTz(_, t, _)
            | DateTimeTzOffset::Time(t)
            | DateTimeTzOffset::TimeTz(t, _) => Some(t),
        }
    }

    #[must_use]
    pub fn tz_offset(&self) -> Option<&TzOffset> {
        match self {
            DateTimeTzOffset::Date(..)
            | DateTimeTzOffset::DateTime(..)
            | DateTimeTzOffset::Time(..) => None,
            DateTimeTzOffset::DateTz(_, tz)
            | DateTimeTzOffset::DateTimeTz(_, _, tz)
            | DateTimeTzOffset::TimeTz(_, tz) => Some(tz),
        }
    }
}

#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    debug_bytes: &'a [u8],
    list_depth: usize,
}
impl<'a> Decoder<'a> {
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            debug_bytes: bytes,
            list_depth: 0,
        }
    }

    /// # Errors
    /// Returns `Err` if the decoder has unconsumed data.
    pub fn close(self) -> Result<(), DecodeError> {
        if self.list_depth != 0 {
            return Err(self.err(ErrorReason::ListCloseNotConsumed));
        }
        if !self.bytes.is_empty() {
            return Err(self.err(ErrorReason::DataNotConsumed));
        }
        Ok(())
    }

    fn err(&self, reason: ErrorReason) -> DecodeError {
        let debug_bytes = self
            .debug_bytes
            .iter()
            .take(30)
            .copied()
            .collect::<Vec<_>>();
        DecodeError {
            reason,
            debug_bytes,
        }
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a bool, or the buffer is empty.
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
        match self.bytes.first().copied() {
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

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not an integer, or the buffer is empty.
    pub fn consume_integer(&mut self) -> Result<i64, DecodeError> {
        let sign = if self.consume_exact(b'-').is_some() {
            -1
        } else {
            1
        };
        let mut seen_underscore = false;
        let mut group_digit_count = 0u16;
        let mut value = 0i64;
        while let Some(b) = self.consume_byte() {
            match b {
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
                    } else if value == 0 {
                        return Err(self.err(ErrorReason::IncorrectDigitGrouping));
                    }
                    seen_underscore = true;
                    group_digit_count = 0;
                }
                _ => break,
            }
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

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not an open list symbol `[`, or the buffer is empty.
    pub fn consume_list_open(&mut self) -> Result<(), DecodeError> {
        self.consume_exact(b'[')
            .ok_or_else(|| self.err(ErrorReason::ExpectedList))?;
        self.list_depth += 1;
        Ok(())
    }

    #[allow(clippy::match_same_arms)]
    fn close_item(&mut self, reason: ErrorReason) -> Result<(), DecodeError> {
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

    fn consume_date_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if b.is_ascii_digit() => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedDate)),
        }
    }

    fn consume_tz_offset_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if b.is_ascii_digit() => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedTimeZoneOffset)),
        }
    }

    fn consume_time_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if b.is_ascii_digit() => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedTime)),
        }
    }

    pub fn has_another_list_item(&mut self) -> bool {
        !matches!(self.bytes.first(), None | Some(&b']'))
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a close list symbol `]`, or the buffer is empty.
    pub fn consume_list_close(&mut self) -> Result<(), DecodeError> {
        if self.list_depth == 0 {
            return Err(self.err(ErrorReason::NotInList));
        }
        self.consume_exact(b']')
            .ok_or_else(|| self.err(ErrorReason::ExpectedListEnd))?;
        self.close_item(ErrorReason::MalformedListEnd)?;
        self.list_depth -= 1;
        Ok(())
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a string, or the buffer is empty.
    #[allow(clippy::missing_panics_doc)]
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

    fn consume_year(&mut self) -> Result<u16, DecodeError> {
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

    fn consume_month(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedMonth))?;
        match self.bytes.first().copied() {
            None => return Err(self.err(ErrorReason::MalformedDate)),
            Some(b'W') => return Err(self.err(ErrorReason::ExpectedMonth)),
            _ => {}
        }
        let d0 = self.consume_date_digit()?;
        let d1 = self.consume_date_digit()?;
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

    fn consume_week(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedWeek))?;
        match self.consume_byte() {
            Some(b'W') => {}
            Some(b'0'..=b'9') => return Err(self.err(ErrorReason::ExpectedWeek)),
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        let d0 = self.consume_date_digit()?;
        let d1 = self.consume_date_digit()?;
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

    fn consume_day(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedDay))?;
        let d0 = self.consume_date_digit()?;
        let d1 = self.consume_date_digit()?;
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

    fn consume_week_day(&mut self) -> Result<u8, DecodeError> {
        self.consume_exact(b'-')
            .ok_or_else(|| self.err(ErrorReason::ExpectedDay))?;
        let d = self.consume_date_digit()?;
        match self.bytes.first().copied() {
            Some(b'T' | b'Z' | b'+' | b'~' | b',' | b']') | None => {}
            _ => return Err(self.err(ErrorReason::MalformedDate)),
        }
        if !(1..=7).contains(&d) {
            return Err(self.err(ErrorReason::DayOutOfRange));
        }
        Ok(d)
    }

    fn consume_date(&mut self) -> Result<Date, DecodeError> {
        enum MonthOrWeek {
            Month(u8),
            Week(u8),
        }
        let y = self.consume_year()?;
        if self.bytes.first().copied() != Some(b'-') {
            return Ok(Date::Year { y });
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
        match month_or_week {
            MonthOrWeek::Month(mo) => {
                let d = self.consume_day()?;
                Ok(Date::YearMonthDay { y, mo, d })
            }
            MonthOrWeek::Week(w) => {
                let d = self.consume_week_day()?;
                Ok(Date::YearWeekDay { y, w, d })
            }
        }
    }

    fn consume_hour(&mut self) -> Result<u8, DecodeError> {
        if self.consume_byte() != Some(b'T') {
            return Err(self.err(ErrorReason::ExpectedHour));
        }
        let d0 = self.consume_time_digit()?;
        let d1 = self.consume_time_digit()?;
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
        let d0 = self.consume_time_digit()?;
        let d1 = self.consume_time_digit()?;
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
        let d0 = self.consume_time_digit()?;
        let d1 = self.consume_time_digit()?;
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

    fn consume_millisecond(&mut self) -> Result<u16, DecodeError> {
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

    fn consume_microsecond(&mut self) -> Result<u32, DecodeError> {
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

    fn consume_nanosecond(&mut self) -> Result<u64, DecodeError> {
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
        if self.bytes.first().copied() != Some(b'.') {
            return Ok(Time::HourMinuteSecond { h, m, s });
        }
        let ms = 1000 * u16::from(s) + self.consume_millisecond()?;
        if self.bytes.first().copied() != Some(b'_') {
            return Ok(Time::HourMinuteMillisecond { h, m, ms });
        }
        let us = 1000 * u32::from(ms) + self.consume_microsecond()?;
        if self.bytes.first().copied() != Some(b'_') {
            return Ok(Time::HourMinuteMicrosecond { h, m, us });
        }
        let ns = 1000 * u64::from(us) + self.consume_nanosecond()?;
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
        let d0 = self.consume_tz_offset_digit()?;
        let d1 = self.consume_tz_offset_digit()?;
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

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a date or time, or the buffer is empty.
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
            Some(Date::Year { .. } | Date::YearMonth { .. } | Date::YearWeek { .. }) => {
                if opt_time.is_some() {
                    return Err(self.err(ErrorReason::MalformedDateTimeTzOffset));
                }
            }
            Some(Date::YearMonthDay { .. } | Date::YearWeekDay { .. }) | None => {}
        }
        let opt_tz_offset = match self.bytes.first() {
            None | Some(&b',' | &b']') => None,
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
}
