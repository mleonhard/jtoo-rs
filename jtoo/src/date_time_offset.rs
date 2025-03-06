#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct DateTimeOffset {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
    pub offset_hour: i8,
    pub offset_minute: u8, // Only one country has used a -00xx timezone, Liberia.  They stopped in 1972.
}

impl DateTimeOffset {
    pub const MIN: Self = Self {
        year: 1,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        nanosecond: 0,
        offset_hour: -23,
        offset_minute: 59,
    };
    pub const MAX: Self = Self {
        year: 9999,
        month: 12,
        day: 31,
        hour: 23,
        minute: 59,
        second: 60,
        nanosecond: 999_999_999,
        offset_hour: 23,
        offset_minute: 59,
    };
    pub const UNIX_EPOCH: Self = Self {
        year: 1970,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        nanosecond: 0,
        offset_hour: 0,
        offset_minute: 0,
    };

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn millisecond(&self) -> u16 {
        u16::try_from(self.nanosecond / 1_000_000).unwrap()
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn microsecond(&self) -> u32 {
        u32::try_from(self.nanosecond / 1_000).unwrap()
    }
}
