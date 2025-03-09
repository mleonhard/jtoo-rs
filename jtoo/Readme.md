jtoo
========
[![crates.io version](https://img.shields.io/crates/v/jtoo.svg)](https://crates.io/crates/jtoo)
[![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/jtoo-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/jtoo-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://github.com/mleonhard/jtoo-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/jtoo-rs/actions)

A Rust library for serializing and deserializing the human-readable [JTOO data format](https://github.com/mleonhard/jtoo-format).

# Features
- `forbid(unsafe_code)`
- Minimal dependencies
- Good test coverage (100%)

# Limitations
- New, not proven in production.
- Todo:
    - `chrono`
    - `HashMap<K,V>`
    - `HashSet<T>`
    - JTOO framing
    - HTOO
    - Improve error messages

# Example
```rust
use jtoo::{Decode, Encode};
use time::OffsetDateTime;

#[derive(Decode, Encode)]
struct ShortMessage {
    pub field0: OffsetDateTime, // Needs "time" feature.
    pub field1: u32,
    pub field2: bool,
}
let text =
  r#"[["field0",D1970-01-01T00:00:00Z],["field1",42],["field2",T]]"#;
let msg = ShortMessage::decode(text).unwrap();
let text2 = msg.encode().unwrap();
assert_eq!(text, &text2);
```

# Performance
[Benchmark](https://github.com/mleonhard/jtoo-rs/blob/b28579f83cad2a72685452bd5d67899445e46b3c/bench/benches/lib.rs) result:
- [serde_json](https://crates.io/crates/serde_json): 169ns
- jtoo: 225ns (33% slower)

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
0/0        0/0          0/0    0/0     0/0      🔒  ├── jtoo_derive 0.1.0
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── jtoo_derive_impl 0.1.0
0/0        14/14        0/0    0/0     3/3      ☢️  │   │   ├── proc-macro2 1.0.94
0/0        4/4          0/0    0/0     0/0      ☢️  │   │   │   └── unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0      ❓  │   │   ├── quote 1.0.39
0/0        14/14        0/0    0/0     3/3      ☢️  │   │   │   └── proc-macro2 1.0.94
0/0        88/88        3/3    0/0     2/2      ☢️  │   │   └── syn 2.0.99
0/0        14/14        0/0    0/0     3/3      ☢️  │   │       ├── proc-macro2 1.0.94
0/0        0/0          0/0    0/0     0/0      ❓  │   │       ├── quote 1.0.39
0/0        4/4          0/0    0/0     0/0      ☢️  │   │       └── unicode-ident 1.0.18
0/0        14/14        0/0    0/0     3/3      ☢️  │   ├── proc-macro2 1.0.94
0/0        88/88        3/3    0/0     2/2      ☢️  │   └── syn 2.0.99
0/0        0/0          0/0    0/0     0/0      🔒  ├── rust_decimal 1.36.0
2/2        340/340      2/2    0/0     7/7      ☢️  │   ├── arrayvec 0.7.6
0/0        0/0          0/0    0/0     0/0      ❓  │   └── num-traits 0.2.19
                                                       │       [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓  │       └── autocfg 1.4.0
2/5        314/342      0/0    0/0     6/6      ☢️  └── time 0.3.39
1/1        4/4          0/0    0/0     1/1      ☢️      ├── deranged 0.3.11
0/0        0/0          0/0    0/0     0/0      ❓      │   ├── num-traits 0.2.19
2/2        29/29        0/0    0/0     0/0      ☢️      │   └── powerfmt 0.2.0
0/0        0/0          0/0    0/0     0/0      ❓      ├── num-conv 0.1.0
2/2        29/29        0/0    0/0     0/0      ☢️      ├── powerfmt 0.2.0
0/0        0/0          0/0    0/0     0/0      ❓      └── time-core 0.1.3

7/10       793/821      5/5    0/0     19/19

```
# Changelog
- v0.1.0 - Initial version.

License: Apache-2.0
