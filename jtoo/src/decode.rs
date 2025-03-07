use crate::decoder::Decoder;
use crate::{escape_ascii, DateTimeOffset};
use core::fmt::Debug;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorReason {
    DataNotConsumed,
    DayOutOfRange,
    DecimalExponentOutOfRange,
    DecimalMantissaOutOfRange,
    DecimalOutOfRange,
    ExpectedBool,
    ExpectedByteString,
    ExpectedDateTimeOffset,
    ExpectedDecimal,
    ExpectedInteger,
    ExpectedList,
    ExpectedListEnd,
    ExpectedListSeparator,
    ExpectedSingleZero,
    ExpectedString,
    ExpectedTimestamp,
    ExpectedUnsignedInteger,
    HourOutOfRange,
    IncompleteEscapeSequence,
    IncorrectDigitGrouping,
    IntegerTooLarge,
    InvalidEscapeSequence,
    ListCloseNotConsumed,
    MalformedBool,
    MalformedByteString,
    MalformedDate,
    MalformedDateTimeOffset,
    MalformedDecimal,
    MalformedInteger,
    MalformedListEnd,
    MalformedOffset,
    MalformedString,
    MalformedTime,
    MalformedTimestamp,
    MinuteOutOfRange,
    MonthOutOfRange,
    NegativeZero,
    NotInList,
    NotUtf8,
    SecondOutOfRange,
    TimestampOutOfRange,
    TimezoneOffsetHourOutOfRange,
    TimezoneOffsetMinuteOutOfRange,
    UnclosedString,
    UppercaseHexNotAllowedInByteString,
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
impl Decode for bool {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_bool()
    }
}
impl Decode for i8 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_integer()?;
        Self::try_from(value).map_err(|_| decoder.err(ErrorReason::IntegerTooLarge))
    }
}
impl Decode for u8 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_unsigned_integer()?;
        Self::try_from(value).map_err(|_| decoder.err(ErrorReason::IntegerTooLarge))
    }
}
impl Decode for i16 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_integer()?;
        Self::try_from(value).map_err(|_| decoder.err(ErrorReason::IntegerTooLarge))
    }
}
impl Decode for u16 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_unsigned_integer()?;
        Self::try_from(value).map_err(|_| decoder.err(ErrorReason::IntegerTooLarge))
    }
}
impl Decode for i32 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_integer()?;
        Self::try_from(value).map_err(|_| decoder.err(ErrorReason::IntegerTooLarge))
    }
}
impl Decode for u32 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_unsigned_integer()?;
        Self::try_from(value).map_err(|_| decoder.err(ErrorReason::IntegerTooLarge))
    }
}
impl Decode for i64 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_integer()
    }
}
impl Decode for u64 {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_unsigned_integer()
    }
}
impl Decode for Box<str> {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let s = decoder.consume_string()?;
        Ok(s.into_boxed_str())
    }
}
impl Decode for String {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_string()
    }
}
impl<T: Decode> Decode for Option<T> {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_list_open()?;
        let value = if decoder.has_another_list_item() {
            Some(T::decode_using(decoder)?)
        } else {
            None
        };
        decoder.consume_list_close()?;
        Ok(value)
    }
}
impl<T: Decode> Decode for Box<[T]> {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let result = <Vec<T>>::decode_using(decoder)?;
        Ok(result.into_boxed_slice())
    }
}
impl<T: Decode> Decode for Vec<T> {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_list_open()?;
        let mut result = Vec::new();
        if decoder.has_another_list_item() {
            let value = T::decode_using(decoder)?;
            result.push(value);
        }
        decoder.consume_list_close()?;
        Ok(result)
    }
}
impl Decode for SystemTime {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let ts = decoder.consume_timestamp_nanoseconds()?;
        SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_nanos(ts))
            .ok_or_else(|| decoder.err(ErrorReason::TimestampOutOfRange))
    }
}
#[cfg(feature = "rust_decimal")]
impl Decode for rust_decimal::Decimal {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let jtoo_decimal = decoder.consume_decimal()?;
        if 28 < jtoo_decimal.neg_exponent {
            return Err(decoder.err(ErrorReason::DecimalExponentOutOfRange));
        }
        Ok(rust_decimal::Decimal::new(
            jtoo_decimal.mantissa,
            u32::from(jtoo_decimal.neg_exponent),
        ))
    }
}
impl Decode for DateTimeOffset {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.consume_date_time_offset()
    }
}
#[cfg(feature = "time")]
impl Decode for time::OffsetDateTime {
    fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let value = decoder.consume_date_time_offset()?;
        let year = i32::from(value.year);
        let month = time::Month::try_from(value.month)
            .map_err(|_| decoder.err(ErrorReason::MalformedDate))?;
        let date = time::Date::from_calendar_date(year, month, value.day)
            .map_err(|_| decoder.err(ErrorReason::MalformedDate))?;
        let time =
            time::Time::from_hms_nano(value.hour, value.minute, value.second, value.nanosecond)
                .map_err(|_| decoder.err(ErrorReason::MalformedTime))?;
        let offset_minute = value.offset_hour.signum()
            * i8::try_from(value.offset_minute)
                .map_err(|_| decoder.err(ErrorReason::MalformedDateTimeOffset))?;
        let offset = time::UtcOffset::from_hms(value.offset_hour, offset_minute, 0)
            .map_err(|_| decoder.err(ErrorReason::MalformedDateTimeOffset))?;
        Ok(time::OffsetDateTime::new_in_offset(date, time, offset))
    }
}
