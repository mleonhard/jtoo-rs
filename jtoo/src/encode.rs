use crate::encoder::Encoder;
use std::time::SystemTime;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub enum EncodeError {
    Empty,
    InvalidDay,
    InvalidHour,
    InvalidMicrosecond,
    InvalidMillisecond,
    InvalidMinute,
    InvalidMonth,
    InvalidNanosecond,
    InvalidOffset,
    InvalidSecond,
    InvalidTimestamp,
    InvalidYear,
    NotInByteString,
    NotInList,
    NotInString,
    OutOfRange,
    UnclosedByteString,
    UnclosedList,
    UnclosedString,
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

impl Encode for bool {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_bool(*self)
    }
}
impl Encode for i8 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::from(*self))
    }
}
impl Encode for u8 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::from(*self))
    }
}
impl Encode for i16 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::from(*self))
    }
}
impl Encode for u16 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::from(*self))
    }
}
impl Encode for i32 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::from(*self))
    }
}
impl Encode for u32 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::from(*self))
    }
}
impl Encode for i64 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(*self)
    }
}
impl Encode for u64 {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_integer(i64::try_from(*self).map_err(|_| EncodeError::OutOfRange)?)
    }
}
impl Encode for &str {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_string(self)
    }
}
impl Encode for Box<str> {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_string(self)
    }
}
impl Encode for String {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.append_string(self)
    }
}
impl<T: Encode> Encode for Option<T> {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.open_list()?;
        if let Some(value) = self {
            value.encode_using(encoder)?;
        }
        encoder.close_list()
    }
}
impl<T: Encode> Encode for &[T] {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.open_list()?;
        for item in *self {
            item.encode_using(encoder)?;
        }
        encoder.close_list()
    }
}
impl<T: Encode> Encode for Box<[T]> {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        self.as_ref().encode_using(encoder)
    }
}
impl<T: Encode> Encode for Vec<T> {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        self.as_slice().encode_using(encoder)
    }
}
impl Encode for SystemTime {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        // A u64 can hold a nanoseconds timestamp up to the year 2554.
        let duration = self
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| EncodeError::OutOfRange)?;
        let nanoseconds = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);
        if nanoseconds % 1000 != 0 {
            return encoder.append_timestamp_nanosecond(nanoseconds);
        }
        let microseconds = nanoseconds / 1000;
        if microseconds % 1000 != 0 {
            return encoder.append_timestamp_microseconds(microseconds);
        }
        let milliseconds = microseconds / 1000;
        if milliseconds % 1000 != 0 {
            return encoder.append_timestamp_milliseconds(milliseconds);
        }
        let seconds = milliseconds / 1000;
        encoder.append_timestamp_seconds(seconds)
    }
}
#[cfg(feature = "rust_decimal")]
impl Encode for rust_decimal::Decimal {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        use rust_decimal::prelude::ToPrimitive;
        let value = self.normalize();
        let mantissa = value.mantissa().to_i64().ok_or(EncodeError::OutOfRange)?;
        let exp =
            i8::try_from(-i64::from(value.fract().scale())).map_err(|_| EncodeError::OutOfRange)?;
        encoder.append_decimal(mantissa, exp)
    }
}
