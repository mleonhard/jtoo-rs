use crate::decimal::Decimal;
use crate::{DateTimeOffset, DecodeError, ErrorReason};

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

    #[must_use]
    pub fn err(&self, reason: ErrorReason) -> DecodeError {
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

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a byte string, or the buffer is empty.
    #[allow(clippy::manual_is_ascii_check)]
    pub fn consume_byte_string(&mut self) -> Result<Vec<u8>, DecodeError> {
        if !self.consume_exact(b'B') {
            return Err(self.err(ErrorReason::ExpectedByteString));
        }
        let mut result = Vec::new();
        loop {
            let d0 = match self.consume_byte() {
                Some(b) if (b'0'..=b'9').contains(&b) => b - b'0',
                Some(b) if (b'a'..=b'f').contains(&b) => 10 + b - b'a',
                Some(b) if (b'A'..=b'F').contains(&b) => {
                    return Err(self.err(ErrorReason::UppercaseHexNotAllowedInByteString))
                }
                _ => break,
            };
            let d1 = match self.consume_byte() {
                Some(b) if (b'0'..=b'9').contains(&b) => b - b'0',
                Some(b) if (b'a'..=b'f').contains(&b) => 10 + b - b'a',
                Some(b) if (b'A'..=b'F').contains(&b) => {
                    return Err(self.err(ErrorReason::UppercaseHexNotAllowedInByteString))
                }
                _ => return Err(self.err(ErrorReason::MalformedByteString)),
            };
            let b = (d0 << 4) | d1;
            result.push(b);
        }
        self.close_item(ErrorReason::MalformedByteString)?;
        Ok(result)
    }

    fn consume_exact(&mut self, c: u8) -> bool {
        if self.bytes.first() == Some(&c) {
            self.bytes = &self.bytes[1..];
            true
        } else {
            false
        }
    }

    fn consume_integer_part(&mut self) -> Result<Option<u64>, DecodeError> {
        let mut seen_underscore = false;
        let mut group_digit_count = 0u16;
        let mut value = 0u64;
        while let Some(b) = self.bytes.first().copied() {
            match b {
                b'0'..=b'9' => {
                    self.consume_byte();
                    group_digit_count += 1;
                    let d = u64::from(b - b'0');
                    value = value
                        .checked_mul(10)
                        .ok_or_else(|| self.err(ErrorReason::IntegerTooLarge))?;
                    value = value
                        .checked_add(d)
                        .ok_or_else(|| self.err(ErrorReason::IntegerTooLarge))?;
                    if value == 0 && 1 < group_digit_count {
                        return Err(self.err(ErrorReason::ExpectedSingleZero));
                    }
                }
                b'_' => {
                    self.consume_byte();
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
            return Ok(None);
        }
        if seen_underscore && group_digit_count != 3 {
            return Err(self.err(ErrorReason::IncorrectDigitGrouping));
        }
        if !seen_underscore && 3 < group_digit_count {
            return Err(self.err(ErrorReason::IncorrectDigitGrouping));
        }
        Ok(Some(value))
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not an integer, or the buffer is empty.
    pub fn consume_integer(&mut self) -> Result<i64, DecodeError> {
        let is_neg = self.consume_exact(b'-');
        let Some(value) = self.consume_integer_part()? else {
            return Err(self.err(ErrorReason::ExpectedInteger));
        };
        self.close_item(ErrorReason::MalformedInteger)?;
        if is_neg {
            if value == 0 {
                Err(self.err(ErrorReason::NegativeZero))
            } else if value == 9_223_372_036_854_775_808 {
                Ok(-9_223_372_036_854_775_808)
            } else {
                let value =
                    i64::try_from(value).map_err(|_| self.err(ErrorReason::IntegerTooLarge))?;
                Ok(value
                    .checked_mul(-1)
                    .ok_or_else(|| self.err(ErrorReason::IntegerTooLarge))?)
            }
        } else {
            i64::try_from(value).map_err(|_| self.err(ErrorReason::IntegerTooLarge))
        }
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not an unsigned integer, or the buffer is empty.
    pub fn consume_unsigned_integer(&mut self) -> Result<u64, DecodeError> {
        let Some(value) = self.consume_integer_part()? else {
            return Err(self.err(ErrorReason::ExpectedUnsignedInteger));
        };
        self.close_item(ErrorReason::MalformedInteger)?;
        Ok(value)
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not an open list symbol `[`, or the buffer is empty.
    pub fn consume_list_open(&mut self) -> Result<(), DecodeError> {
        if !self.consume_exact(b'[') {
            return Err(self.err(ErrorReason::ExpectedList));
        }
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

    fn consume_offset_digit(&mut self) -> Result<u8, DecodeError> {
        match self.consume_byte() {
            Some(b) if b.is_ascii_digit() => Ok(b - b'0'),
            _ => Err(self.err(ErrorReason::MalformedOffset)),
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
        if !self.consume_exact(b']') {
            return Err(self.err(ErrorReason::ExpectedListEnd));
        }
        self.close_item(ErrorReason::MalformedListEnd)?;
        self.list_depth -= 1;
        Ok(())
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a string, or the buffer is empty.
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_lines)]
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

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a decimal, or the buffer is empty.
    #[allow(clippy::too_many_lines)]
    pub fn consume_decimal(&mut self) -> Result<Decimal, DecodeError> {
        enum FormatChecker {
            Start0,
            Start1,
            Start2,
            Start3,
            Left0,
            Left1,
            Left2,
            Left3,
            Right0,
            Right1,
            Right2,
            Right3,
        }
        #[allow(clippy::match_same_arms)]
        impl FormatChecker {
            pub fn digit(&mut self) -> Result<(), ErrorReason> {
                *self = match self {
                    FormatChecker::Start0 => FormatChecker::Start1,
                    FormatChecker::Start1 => FormatChecker::Start2,
                    FormatChecker::Start2 => FormatChecker::Start3,
                    FormatChecker::Start3 => return Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Left0 => FormatChecker::Left1,
                    FormatChecker::Left1 => FormatChecker::Left2,
                    FormatChecker::Left2 => FormatChecker::Left3,
                    FormatChecker::Left3 => return Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Right0 => FormatChecker::Right1,
                    FormatChecker::Right1 => FormatChecker::Right2,
                    FormatChecker::Right2 => FormatChecker::Right3,
                    FormatChecker::Right3 => return Err(ErrorReason::IncorrectDigitGrouping),
                };
                Ok(())
            }
            pub fn underscore(&mut self) -> Result<(), ErrorReason> {
                *self = match self {
                    FormatChecker::Start0 => return Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Start1 | FormatChecker::Start2 | FormatChecker::Start3 => {
                        FormatChecker::Left0
                    }
                    FormatChecker::Left0 | FormatChecker::Left1 | FormatChecker::Left2 => {
                        return Err(ErrorReason::IncorrectDigitGrouping)
                    }
                    FormatChecker::Left3 => FormatChecker::Left0,
                    FormatChecker::Right0 | FormatChecker::Right1 | FormatChecker::Right2 => {
                        return Err(ErrorReason::IncorrectDigitGrouping)
                    }
                    FormatChecker::Right3 => FormatChecker::Right0,
                };
                Ok(())
            }
            pub fn period(&mut self) -> Result<(), ErrorReason> {
                *self = match self {
                    FormatChecker::Start0 => return Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Start1 | FormatChecker::Start2 | FormatChecker::Start3 => {
                        FormatChecker::Right0
                    }
                    FormatChecker::Left0 | FormatChecker::Left1 | FormatChecker::Left2 => {
                        return Err(ErrorReason::IncorrectDigitGrouping)
                    }
                    FormatChecker::Left3 => FormatChecker::Right0,
                    FormatChecker::Right0
                    | FormatChecker::Right1
                    | FormatChecker::Right2
                    | FormatChecker::Right3 => return Err(ErrorReason::MalformedDecimal),
                };
                Ok(())
            }
            pub fn finish(&mut self) -> Result<(), ErrorReason> {
                match self {
                    FormatChecker::Start0
                    | FormatChecker::Start1
                    | FormatChecker::Start2
                    | FormatChecker::Start3 => Err(ErrorReason::MalformedDecimal),
                    FormatChecker::Left0 => Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Left1 => Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Left2 => Err(ErrorReason::IncorrectDigitGrouping),
                    FormatChecker::Left3 => Err(ErrorReason::MalformedDecimal),
                    FormatChecker::Right0 => Err(ErrorReason::MalformedDecimal),
                    FormatChecker::Right1 => Ok(()),
                    FormatChecker::Right2 => Ok(()),
                    FormatChecker::Right3 => Ok(()),
                }
            }
        }
        let mut format_checker = FormatChecker::Start0;
        let is_neg = self.consume_exact(b'-');
        let mut seen_period = false;
        let mut mantissa = 0u64;
        let mut last_digit = 0u64;
        let mut neg_exponent = 0u8;
        while let Some(b) = self.bytes.first().copied() {
            match b {
                b'0'..=b'9' => {
                    self.consume_byte();
                    format_checker.digit().map_err(|reason| self.err(reason))?;
                    mantissa = mantissa
                        .checked_mul(10)
                        .ok_or_else(|| self.err(ErrorReason::DecimalMantissaOutOfRange))?;
                    mantissa = mantissa
                        .checked_add(last_digit)
                        .ok_or_else(|| self.err(ErrorReason::DecimalMantissaOutOfRange))?;
                    if seen_period {
                        neg_exponent = neg_exponent
                            .checked_add(1)
                            .ok_or_else(|| self.err(ErrorReason::DecimalExponentOutOfRange))?;
                    }
                    last_digit = u64::from(b - b'0');
                }
                b'_' => {
                    self.consume_byte();
                    format_checker
                        .underscore()
                        .map_err(|reason| self.err(reason))?;
                }
                b'.' => {
                    format_checker.period().map_err(|reason| self.err(reason))?;
                    self.consume_byte();
                    seen_period = true;
                }
                _ => break,
            }
        }
        format_checker.finish().map_err(|reason| self.err(reason))?;
        if last_digit == 0 && neg_exponent == 1 {
            neg_exponent = 0;
        } else {
            mantissa = mantissa
                .checked_mul(10)
                .ok_or_else(|| self.err(ErrorReason::DecimalMantissaOutOfRange))?;
            mantissa = mantissa
                .checked_add(last_digit)
                .ok_or_else(|| self.err(ErrorReason::DecimalMantissaOutOfRange))?;
        }
        self.close_item(ErrorReason::MalformedDecimal)?;
        if is_neg {
            match mantissa {
                0 => return Err(self.err(ErrorReason::NegativeZero)),
                9_223_372_036_854_775_808 => {
                    return Ok(Decimal::new(-9_223_372_036_854_775_808, neg_exponent))
                }
                _ => {}
            }
        }
        let mantissa = i64::try_from(mantissa)
            .map_err(|_| self.err(ErrorReason::DecimalMantissaOutOfRange))?;
        let mantissa = mantissa
            .checked_mul(if is_neg { -1 } else { 1 })
            .ok_or_else(|| self.err(ErrorReason::DecimalMantissaOutOfRange))?;
        Ok(Decimal::new(mantissa, neg_exponent))
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a `DateTimeOffset`, or the buffer is empty.
    pub fn consume_date_time_offset(&mut self) -> Result<DateTimeOffset, DecodeError> {
        if self.consume_byte() != Some(b'D') {
            return Err(self.err(ErrorReason::ExpectedDateTimeOffset));
        }
        let d0 = u16::from(self.consume_date_digit()?);
        let d1 = u16::from(self.consume_date_digit()?);
        let d2 = u16::from(self.consume_date_digit()?);
        let d3 = u16::from(self.consume_date_digit()?);
        let year = 1000 * d0 + 100 * d1 + 10 * d2 + d3;
        if !(1..=9999).contains(&year) {
            return Err(self.err(ErrorReason::YearOutOfRange));
        }
        if self.consume_byte() != Some(b'-') {
            return Err(self.err(ErrorReason::MalformedDate));
        }
        let d0 = self.consume_date_digit()?;
        let d1 = self.consume_date_digit()?;
        let month = 10 * d0 + d1;
        if !(1..=12).contains(&month) {
            return Err(self.err(ErrorReason::MonthOutOfRange));
        }
        if self.consume_byte() != Some(b'-') {
            return Err(self.err(ErrorReason::MalformedDate));
        }
        let d0 = self.consume_date_digit()?;
        let d1 = self.consume_date_digit()?;
        let day = 10 * d0 + d1;
        if !(1..=31).contains(&day) {
            return Err(self.err(ErrorReason::DayOutOfRange));
        }
        if self.consume_byte() != Some(b'T') {
            return Err(self.err(ErrorReason::MalformedDateTimeOffset));
        }
        let d0 = self.consume_time_digit()?;
        let d1 = self.consume_time_digit()?;
        let hour = 10 * d0 + d1;
        if !(0..=23).contains(&hour) {
            return Err(self.err(ErrorReason::HourOutOfRange));
        }
        if self.consume_byte() != Some(b':') {
            return Err(self.err(ErrorReason::MalformedTime));
        }
        let d0 = self.consume_time_digit()?;
        let d1 = self.consume_time_digit()?;
        let minute = 10 * d0 + d1;
        if !(0..=59).contains(&minute) {
            return Err(self.err(ErrorReason::MinuteOutOfRange));
        }
        if self.consume_byte() != Some(b':') {
            return Err(self.err(ErrorReason::MalformedTime));
        }
        let d0 = self.consume_time_digit()?;
        let d1 = self.consume_time_digit()?;
        let second = 10 * d0 + d1;
        if !(0..=60).contains(&second) {
            return Err(self.err(ErrorReason::SecondOutOfRange));
        }
        let nanosecond = self.consume_nanosecond()?;
        let (offset_hour, offset_minute) = self.consume_offset()?;
        self.close_item(ErrorReason::MalformedDateTimeOffset)?;
        Ok(DateTimeOffset {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
            offset_hour,
            offset_minute,
        })
    }

    fn consume_nanosecond(&mut self) -> Result<u32, DecodeError> {
        if self.bytes.first().copied() == Some(b'.') {
            self.consume_byte();
        } else {
            return Ok(0);
        }
        let d0 = u16::from(self.consume_time_digit()?);
        let d1 = u16::from(self.consume_time_digit()?);
        let d2 = u16::from(self.consume_time_digit()?);
        let millisecond = u32::from(100 * d0 + 10 * d1 + d2);
        if self.bytes.first() == Some(&b'_') {
            self.consume_byte();
        } else {
            return Ok(1_000_000 * millisecond);
        }
        let d0 = u16::from(self.consume_time_digit()?);
        let d1 = u16::from(self.consume_time_digit()?);
        let d2 = u16::from(self.consume_time_digit()?);
        let microsecond = u32::from(100 * d0 + 10 * d1 + d2);
        if self.bytes.first() == Some(&b'_') {
            self.consume_byte();
        } else {
            return Ok(1_000_000 * millisecond + 1_000 * microsecond);
        }
        let d0 = u32::from(self.consume_time_digit()?);
        let d1 = u32::from(self.consume_time_digit()?);
        let d2 = u32::from(self.consume_time_digit()?);
        let nanosecond = 100 * d0 + 10 * d1 + d2;
        Ok(1_000_000 * millisecond + 1_000 * microsecond + nanosecond)
    }

    fn consume_offset(&mut self) -> Result<(i8, u8), DecodeError> {
        let sign = match self.consume_byte() {
            Some(b'Z') => {
                return Ok((0, 0));
            }
            Some(b'+') => 1,
            Some(b'-') => -1,
            _ => return Err(self.err(ErrorReason::MalformedDateTimeOffset)),
        };
        let d0 = i8::try_from(self.consume_offset_digit()?).unwrap();
        let d1 = i8::try_from(self.consume_offset_digit()?).unwrap();
        let offset_hour = sign * (10 * d0 + d1);
        if !(-23..=23).contains(&offset_hour) {
            return Err(self.err(ErrorReason::TimezoneOffsetHourOutOfRange));
        }
        match self.bytes.first().copied() {
            Some(b) if b.is_ascii_digit() => {}
            _ => {
                if offset_hour == 0 {
                    return Err(self.err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ));
                }
                return Ok((offset_hour, 0));
            }
        }
        let d0 = self.consume_offset_digit()?;
        let d1 = self.consume_offset_digit()?;
        let offset_minute = 10 * d0 + d1;
        if !(0..=59).contains(&offset_minute) {
            Err(self.err(ErrorReason::TimezoneOffsetMinuteOutOfRange))
        } else if offset_hour == 0 && offset_minute == 0 {
            Err(self.err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ))
        } else if offset_minute == 0 {
            Err(self.err(ErrorReason::ZeroTimeZoneMinutesShouldBeOmitted))
        } else {
            Ok((offset_hour, offset_minute))
        }
    }

    /// # Errors
    /// Returns `Err` when the next item in the buffer is not a timestamp, or the buffer is empty.
    pub fn consume_timestamp_nanoseconds(&mut self) -> Result<u64, DecodeError> {
        if !self.consume_exact(b'S') {
            return Err(self.err(ErrorReason::ExpectedTimestamp));
        }
        let Some(value) = self.consume_integer_part()? else {
            return Err(self.err(ErrorReason::MalformedTimestamp));
        };
        let nanosecond = u64::from(self.consume_nanosecond()?);
        let value = value
            .checked_mul(1_000_000_000)
            .ok_or_else(|| self.err(ErrorReason::TimestampOutOfRange))?;
        let value = value
            .checked_add(nanosecond)
            .ok_or_else(|| self.err(ErrorReason::TimestampOutOfRange))?;
        self.close_item(ErrorReason::MalformedTimestamp)?;
        Ok(value)
    }
}
