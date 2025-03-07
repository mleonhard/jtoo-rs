use std::fmt::{Display, Formatter};

/// A decimal number stored in base-10: `mantissa x 10^exponent`.
/// Supports 18 significant digits and exponents from 0 to -255.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Decimal {
    /// The significand or coefficient.  In the number `123x10^-4`, this is `123`.
    pub mantissa: i64,
    /// Base-10 negative exponent.  In the number `123x10^-4`, this is `4`.
    pub neg_exponent: u8,
}

impl Decimal {
    /// 9223372036854775807 x 10^-0
    pub const MAX: Self = Self {
        mantissa: i64::MAX,
        neg_exponent: 0,
    };
    /// -9223372036854775808 x 10^-0
    pub const MIN: Self = Self {
        mantissa: i64::MIN,
        neg_exponent: 0,
    };
    pub const ZERO: Self = Self {
        mantissa: 0,
        neg_exponent: 0,
    };

    /// `mantissa` is the significand or coefficient.
    /// `neg_exponent` is the base-10 exponent.
    ///
    /// Example: `0.0123 = 123x10^-4`, `mantissa=123`, `neg_exponent=4`
    #[must_use]
    pub fn new(mantissa: i64, neg_exponent: u8) -> Self {
        Self {
            mantissa,
            neg_exponent,
        }
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x10^-{}", self.mantissa, self.neg_exponent)
    }
}
