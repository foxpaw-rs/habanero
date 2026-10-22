//! # Habanero
//! Habanero is a framework for sending and receiving HTTP requests via a
//! client and server without any complex boilerplate or networking required.
//!
//! The Habanero ecosystem leverages two main types to connect to the outside
//! world, the [`Client`] and [`Server`]. Both of these provide simple 
//! interaction methods for quick actions, and generic methods for more complex
//! situations.
//!
//! See the _`docs/`_ directory for additional documentation and usage 
//! examples.
//!
//! ## Architecture
//! The basic philosophy behind Habanero is to make HTTP interaction as simple
//! as possible, with the ability to perform complex tasks if required. The 
//! [`Client`] and [`Server`] types are the entry point to interacting with the
//! crate.
//!
//! Majority of the imports can be brought in to scope with an import of the
//! prelude module. Note that this will not bring into scope any of the error
//! types supplied by the crate. These can all be found in the `err` module
//! ```rust
//!
//! use foxpaw_habanero::prelude::*;
//! use foxpaw_habanero::err::*;
//! ```
//!
//! ## Examples
//! ### Start a Server
//! ```rust, no_run
//! use foxpaw_habanero::prelude::*;
//! use foxpaw_habanero::err::{HttpError, ServerError};
//! 
//! fn main() -> Result<(), ServerError> {
//!     let server = Server::bind("127.0.0.1:7878")?;
//!     server.run(|req: Request| {
//!         if req.uri == "/" {
//!             Ok(Response::new(Version::Http11, 200)
//!                 .with_html("<html><body><h1>Hello World!</h1></body></html>"))
//!         } else {
//!             Err(HttpError::NotFound)
//!         }
//!     });
//!     Ok(())
//! }
//! ```
//! 
//! ### Send a Request to a Remote Host
//! ```rust, no_run
//! use foxpaw_habanero::prelude::*;
//! use foxpaw_habanero::err::ClientError;
//! 
//! fn main() -> Result<(), ClientError> {
//!     let client = Client::new();
//!     let respone = client.get(&"127.0.0.1:7878".parse()?)?;
//!     Ok(())
//! }
//! ```

#![deny(
    clippy::all,
    //clippy::cargo, Todo: Cargo.toml
    clippy::pedantic,
)]

pub mod client;
pub mod codec;
pub mod common;
pub mod connection;
pub mod err;
pub mod router;
pub mod server;
pub mod transport;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::common::{Headers, Method, Request, Response, Scheme, Url, Version};
    pub use crate::router::Router;
    pub use crate::server::{Handler, Server};
}

#[cfg(test)]
mod mock;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;