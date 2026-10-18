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
pub mod connection;
pub mod err;
mod router;
mod server;
pub mod transport;

pub use common::{Headers, Method, Request, Response, Scheme, Url, Version};
pub use router::Router;
pub use server::{Handler, Server};

#[cfg(test)]
mod mock;
