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

#[derive(Debug, Eq, PartialEq)]
pub enum PackError {
    NotInByteString,
    NotInList,
    NotInString,
    UnclosedByteString,
    UnclosedList,
    UnclosedString,
}

pub trait Pack {
    fn pack(&self) -> Result<String, PackError>;
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Elem {
    String,
    ByteString,
    EmptyList,
    List,
}

pub struct Packer {
    stack: Vec<Elem>,
    string: String,
}
impl Packer {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            string: String::new(),
        }
    }

    fn prepare_for_new_value(&mut self) -> Result<(), PackError> {
        match self.stack.last() {
            Some(Elem::String) => Err(PackError::UnclosedString),
            Some(Elem::ByteString) => Err(PackError::UnclosedByteString),
            Some(Elem::EmptyList) => {
                self.stack.pop();
                self.stack.push(Elem::List);
                Ok(())
            }
            None => Ok(()),
            Some(Elem::List) => {
                self.string.push(',');
                Ok(())
            }
        }
    }

    pub fn append_bool(&mut self, b: bool) -> Result<(), PackError> {
        self.prepare_for_new_value()?;
        self.string.push(if b { 'T' } else { 'F' });
        Ok(())
    }

    pub fn open_byte_string(&mut self) -> Result<(), PackError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::ByteString);
        self.string.push('B');
        Ok(())
    }

    pub fn append_byte_string(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        #[inline]
        fn to_hex(b: u8) -> char {
            match b {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => unreachable!(),
            }
        }
        if self.stack.last() == Some(&Elem::ByteString) {
            for b in bytes {
                self.string.push(to_hex(b >> 4));
                self.string.push(to_hex(b & 0x0F));
            }
            Ok(())
        } else {
            Err(PackError::NotInString)
        }
    }

    pub fn close_byte_string(&mut self) -> Result<(), PackError> {
        if self.stack.last() == Some(&Elem::ByteString) {
            self.stack.pop();
            Ok(())
        } else {
            Err(PackError::NotInString)
        }
    }

    pub fn open_list(&mut self) -> Result<(), PackError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::EmptyList);
        self.string.push('[');
        Ok(())
    }

    pub fn close_list(&mut self) -> Result<(), PackError> {
        match self.stack.last() {
            Some(&Elem::EmptyList) | Some(&Elem::List) => {
                self.string.push(']');
                self.stack.pop();
                Ok(())
            }
            _ => Err(PackError::NotInList),
        }
    }

    pub fn open_string(&mut self) -> Result<(), PackError> {
        self.prepare_for_new_value()?;
        self.stack.push(Elem::String);
        self.string.push('"');
        Ok(())
    }

    pub fn append_string(&mut self, s: &str) -> Result<(), PackError> {
        if self.stack.last() == Some(&Elem::String) {
            self.string.push_str(s);
            Ok(())
        } else {
            Err(PackError::NotInString)
        }
    }

    pub fn close_string(&mut self) -> Result<(), PackError> {
        if self.stack.last() == Some(&Elem::String) {
            self.stack.pop();
            self.string.push('"');
            Ok(())
        } else {
            Err(PackError::NotInString)
        }
    }

    // TODO: Implement integer, decimal, and times.

    pub fn to_string(mut self) -> Result<String, PackError> {
        match self.stack.pop() {
            Some(Elem::String) => Err(PackError::UnclosedString),
            Some(Elem::ByteString) => Err(PackError::UnclosedByteString),
            Some(Elem::EmptyList) | Some(Elem::List) => Err(PackError::UnclosedList),
            None => Ok(self.string),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnpackError {}

pub struct Unpacker<'a> {
    bytes: &'a [u8],
}
impl<'a> Unpacker<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

pub trait Unpack {
    fn unpack(bytes: &[u8]) -> Result<Self, UnpackError>
    where
        Self: Sized;
}
