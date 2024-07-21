//! jtoo
//! ========
//! [![crates.io version](https://img.shields.io/crates/v/jtoo.svg)](https://crates.io/crates/jtoo)
//! [![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/jtoo/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/jtoo/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://github.com/mleonhard/jtoo/workflows/CI/badge.svg)](https://github.com/mleonhard/jtoo/actions)
//!
//! A Rust library for serializing and deserializing the [JTOO data format](https://github.com/mleonhard/jtoo-format).
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - No dependencies
//! - Good test coverage (NN%)
//!
//! # Limitations
//! - New, not proven in production.
//! - To do:
//!     - Pack trait
//!     - Unpack trait
//!     - Derive Pack trait
//!     - Derive Unpack trait
//!     - Support interned strings
//!     - Support aliases
//!     - Support enums and type discrimination
//!
//! Simple example:
//! ```rust
//! // TODO: Fix example.
//! //use jtoo::{Unpack};
//! //
//! //#[derive(Unpack)]
//! //struct Message {
//! //    name: String,
//! //}
//! //fn parse(message_bytes: &[u8]) -> Result<Message, String> {
//! //    let message = Message::unpack(message_bytes)
//! //        .map_err(|e| format!("error processing {} byte message: {e}", message_bytes.len()))?;
//! //    return message;
//! //}
//! ```
//! # Cargo Geiger Safety Report
//! # Alternatives
//! - [serde_json](https://crates.io/crates/serde_json)
//!
//! # Changelog
//! - v0.1.0 - Initial version.
#![forbid(unsafe_code)]

mod decode;
mod encode;

pub use decode::*;
pub use encode::*;
