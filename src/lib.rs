//! Habanero
//!
//! Todo: Library documentation

#![deny(
    clippy::all,
    //clippy::cargo, Todo: Cargo.toml
    clippy::pedantic,
)]
#![allow(clippy::wildcard_imports)]

pub mod codec;
mod common;
pub mod transport;

pub use common::*;
