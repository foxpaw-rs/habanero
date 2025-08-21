//! # Habanero
//! HTTP Client and Server ecosystem for Rust.

//! Todo(Paul): Library documentation

#[deny(
    // TODO(Paul): Uncomment when the cargo.toml file is finished.
    // clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
)]
pub mod request;

pub use request::{Request, Verb};
