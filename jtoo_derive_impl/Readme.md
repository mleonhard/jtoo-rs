This crate implements the derive macros for the [`jtoo`](https://crates.io/crates/jtoo)
`Decode` and `Encode` traits.

# Development

Inspect macro expansion:
1. Comment out tests except one
2. `cargo expand --package jtoo --test struct_test |less`

License: MIT OR Apache-2.0
