// Conditionally use include_str, for Rust < 1.54.
// See: https://github.com/rust-lang/rust/issues/82768#issuecomment-803935643
#![cfg_attr(feature = "doc", feature(extended_key_value_attributes))]
#![cfg_attr(feature = "doc", cfg_attr(feature = "doc", doc = include_str!("../README.md")))]

pub mod env_vars;
