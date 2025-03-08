//! ```
//! jtoo-rs % cargo +nightly bench --package bench
//!    Compiling jtoo v0.1.0 (/Users/user/jtoo-rs/jtoo)
//!    Compiling bench v0.0.0 (/Users/user/jtoo-rs/bench)
//!     Finished `bench` profile [optimized] target(s) in 0.51s
//!      Running benches/lib.rs (target/release/deps/lib-a8a3b18865cf144a)
//!
//! running 2 tests
//! test short_message_json ... bench:         168.76 ns/iter (+/- 8.25)
//! test short_message_jtoo ... bench:         224.53 ns/iter (+/- 7.51)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out; finished in 3.02s
//! ```
#![allow(soft_unstable)]
#![feature(test)]
#![forbid(unsafe_code)]
extern crate test;

use jtoo::Decode;
use serde::Deserialize;
use test::Bencher;
use time::serde::iso8601;
use time::OffsetDateTime;

#[derive(Deserialize, Decode)]
#[allow(dead_code)]
struct ShortMessage {
    #[serde(with = "iso8601")]
    pub field0: OffsetDateTime,
    pub field1: String,
    pub field2: u32,
    pub field3: Vec<String>,
}
impl ShortMessage {
    const JSON: &'static [u8] =
        br#"{"field0":"1970-01-01T00:00:00Z","field1":"string1","field2":2,"field3":["a","b","c"]}"#;
    const JTOO: &'static [u8] =
        br#"[["field0",D1970-01-01T00:00:00Z],["field1","string1"],["field2",2],["field3",["a","b","c"]]]"#;
}

#[bench]
fn short_message_json(b: &mut Bencher) {
    b.iter(|| {
        let _: ShortMessage = serde_json::from_slice(ShortMessage::JSON).unwrap();
    });
}

#[bench]
fn short_message_jtoo(b: &mut Bencher) {
    b.iter(|| {
        let _ = ShortMessage::decode(ShortMessage::JTOO).unwrap();
    });
}
