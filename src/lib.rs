//! Habanero
//!
//! Todo: Library documentation

#![deny(
    clippy::all,
    //clippy::cargo, Todo: Cargo.toml
    clippy::pedantic,
)]

mod client;
pub mod codec;
mod common;
pub mod connection;
pub mod err;
mod router;
mod server;
pub mod transport;

pub use client::Client;
pub use common::{Headers, Method, Request, Response, Scheme, Url, Version};
pub use router::Router;
pub use server::{Handler, Server};

#[cfg(test)]
mod mock;
