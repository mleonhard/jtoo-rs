use crate::decoder::Decoder;
use crate::escape_ascii;
use core::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorReason {
    DataNotConsumed,
    DayOutOfRange,
    ExpectedBool,
    ExpectedByteString,
    ExpectedDateTimeOffset,
    ExpectedInteger,
    ExpectedList,
    ExpectedListEnd,
    ExpectedListSeparator,
    ExpectedSingleZero,
    ExpectedString,
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
    MalformedInteger,
    MalformedListEnd,
    MalformedOffset,
    MalformedString,
    MalformedTime,
    MinuteOutOfRange,
    MonthOutOfRange,
    NegativeZero,
    NotInList,
    NotUtf8,
    SecondOutOfRange,
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
