//! Habanero
//!
//! Todo: Library documentation

#![deny(
    clippy::all,
    //clippy::cargo, Todo: Cargo.toml
    clippy::pedantic,
)]

pub mod codec;
mod common;
pub mod err;
pub mod transport;

pub use common::{Headers, Method, Request, Response, Scheme, Url, Version};
