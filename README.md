jtoo
========
[![crates.io version](https://img.shields.io/crates/v/jtoo.svg)](https://crates.io/crates/jtoo)
[![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/jtoo/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/jtoo/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://github.com/mleonhard/jtoo/workflows/CI/badge.svg)](https://github.com/mleonhard/jtoo/actions)

A Rust library for serializing and deserializing the [JTOO data format](https://github.com/mleonhard/jtoo-format).

# Features
- `forbid(unsafe_code)`
- No dependencies
- Good test coverage (NN%)

# Limitations
- New, not proven in production.
- To do:
    - Pack trait
    - Unpack trait
    - Derive Pack trait
    - Derive Unpack trait
    - Support interned strings
    - Support aliases
    - Support enums and type discrimination

Simple example:
```rust
use jtoo::{Unpack, unpack};

#[derive(Deserialize)]
struct Message {
    name: String,
}
fn parse(message_bytes: &[u8]) -> Result<Message, String> {
    let message: Message = unpack(message_bytes)
        .map_err(|e| format!("error processing {} byte message: {e}", message_bytes.len()))?;
    return message;
}
```
# Cargo Geiger Safety Report
```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒  jtoo 0.1.0

0/0        0/0          0/0    0/0     0/0    

```
# Alternatives
- [serde_json](https://crates.io/crates/serde_json)

# Changelog
- v0.1.0 - Initial version.

License: MIT OR Apache-2.0
