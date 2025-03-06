use std::fmt::{Display, Formatter};

/// A decimal number stored in base-10 scientific notation: `mantissa x 10^exponent`.
/// Supports 18 significant digits and exponents from -128 to 127.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Decimal {
    /// The significand or coefficient.  In the number `123x10^-4`, this is `123`.
    pub mantissa: i64,
    /// Base-10 exponent.  In the number `123x10^-4`, this is `-4`.
    pub exponent: i8,
}

impl Decimal {
    /// 9223372036854775807 x 10^127
    pub const MAX: Self = Self {
        mantissa: i64::MAX,
        exponent: i8::MAX,
    };
    /// -9223372036854775808 x 10^127
    pub const MIN: Self = Self {
        mantissa: i64::MIN,
        exponent: i8::MAX,
    };
    pub const ZERO: Self = Self {
        mantissa: 0,
        exponent: 0,
    };

    /// `mantissa` is the significand or coefficient.
    /// `exponent` is the base-10 exponent.
    ///
    /// Example: `0.0123 = 123x10^-4`, `mantissa=123`, `exponent=-4`
    #[must_use]
    pub fn new(mantissa: i64, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x10^{}", self.mantissa, self.exponent)
    }
}
