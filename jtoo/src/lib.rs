//! jtoo
//! ========
//! [![crates.io version](https://img.shields.io/crates/v/jtoo.svg)](https://crates.io/crates/jtoo)
//! [![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/jtoo-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/jtoo-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://github.com/mleonhard/jtoo-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/jtoo-rs/actions)
//!
//! A Rust library for serializing and deserializing the human-readable [JTOO data format](https://github.com/mleonhard/jtoo-format).
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - Minimal dependencies
//! - Good test coverage (100%)
//!
//! # Limitations
//! - New, not proven in production.
//! - Todo:
//!     - `chrono`
//!     - `HashMap<K,V>`
//!     - `HashSet<T>`
//!     - JTOO framing
//!     - HTOO
//!     - Improve error messages
//!
//! # Example
//! ```rust
//! # #[cfg(feature = "derive")] {
//! use jtoo::{Decode, Encode};
//! use time::OffsetDateTime;
//!
//! #[derive(Decode, Encode)]
//! struct ShortMessage {
//!     pub field0: OffsetDateTime,
//!     pub field1: u32,
//!     pub field2: bool,
//! }
//! let text =
//!   r#"[["field0",D1970-01-01T00:00:00Z],["field1",42],["field2",T]]"#;
//! let msg = ShortMessage::decode(text).unwrap();
//! let text2 = msg.encode().unwrap();
//! assert_eq!(text, &text2);
//! # }
//! ```
//!
//! # Performance
//! [Benchmark](https://github.com/mleonhard/jtoo-rs/blob/b28579f83cad2a72685452bd5d67899445e46b3c/bench/benches/lib.rs) result:
//! - [serde_json](https://crates.io/crates/serde_json): 169ns
//! - jtoo: 225ns (33% slower)
//!
//! # Cargo Geiger Safety Report
//!
//! # Changelog
//! - v0.1.0 - Initial version.
#![forbid(unsafe_code)]

mod date_time_offset;
mod decimal;
mod decode;
mod decoder;
mod encode;
mod encoder;

pub use date_time_offset::*;
pub use decimal::*;
pub use decode::*;
pub use decoder::*;
pub use encode::*;
pub use encoder::*;

#[cfg(feature = "derive")]
pub use jtoo_derive::*;

/// Converts a byte slice into a string using
/// [`core::ascii::escape_default`](https://doc.rust-lang.org/core/ascii/fn.escape_default.html)
/// to escape each byte.
///
/// # Example
/// ```
/// use jtoo::escape_ascii;
/// assert_eq!("abc", escape_ascii(b"abc"));
/// assert_eq!("abc\\n", escape_ascii(b"abc\n"));
/// assert_eq!(
///     "Euro sign: \\xe2\\x82\\xac",
///     escape_ascii("Euro sign: \u{20AC}".as_bytes())
/// );
/// assert_eq!("\\x01\\x02\\x03", escape_ascii(&[1, 2, 3]));
/// ```
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn escape_ascii(input: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for byte in input.as_ref() {
        for ascii_byte in core::ascii::escape_default(*byte) {
            result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
        }
    }
    result
}

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ByteString(pub Vec<u8>);
