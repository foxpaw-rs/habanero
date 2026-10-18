//! Server
//!
//! This module houses the `Server` struct to run the HTTP server, along with
//! the [`Handler`] trait it will run over.
//!
//! # Architecture
//! There are two types exposed by this module:
//! * [`Handler`]: A trait that the [`Server`] uses to be run over, which
//!   handles an incoming [`Request`] and returns either a [`Response`] or an
//!   [`HttpError`]; and
//! * [`Server`]: The actual server type, used to bind to an address and run
//!   over incoming HTTP requests.
//!
//! # Examples
//! ```rust, no_run
//! use foxpaw_habanero::*;
//! use foxpaw_habanero::err::{HttpError, ServerError};
//!
//! fn start_server() -> Result<(), ServerError> {
//!     let server = Server::bind("127.0.0.1:8080")?;
//!     server.run(|req: Request| {
//!         if req.method == Method::Get {
//!             Ok(Response::new(Version::Http11, 200))
//!         } else {
//!             Err(HttpError::NotFound)
//!         }
//!     });
//!     Ok(())
//! }
//! ```

use crate::common::{Request, Response, Version};
use crate::connection::{Http1Server, Server as ServerConnection};
use crate::err::{HttpError, ServerError};
use crate::transport::Tcp;
use std::io;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::Arc;
use std::thread;

/// `Handler`
///
/// A handler handles a [`Request`], performs some sort of logic and generates
/// a [`Response`] to be returned.
pub trait Handler: Send + Sync {
    /// Handle
    ///
    /// Handle an incoming [`Request`] and return a [`Response`].
    ///
    /// # Errors
    /// This method can error for any reason by the implementing busines logic.
    /// It is the responsibility of the [`Server`] to try and recover from
    /// these if possible.
    fn handle(&self, req: Request) -> Result<Response, HttpError>;
}

impl<F> Handler for F
where
    F: Fn(Request) -> Result<Response, HttpError> + Send + Sync,
{
    /// Handle
    ///
    /// Handle an incoming [`Request`] and return a [`Response`]. This generic
    /// implementation allows any method accepting a [`Request`] and returning
    /// a [`Response`] to be used as a single [`Handler`] without using the
    /// full [`Router`].
    ///
    /// # Errors
    /// This method can error for any reason by the implementing busines logic.
    /// It is the responsibility of the [`Server`] to try and recover from
    /// these if possible.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::{HttpError, ServerError};
    ///
    /// fn start_server() -> Result<(), ServerError> {
    ///     let server = Server::bind("127.0.0.1:8080")?;
    ///     server.run(|req: Request| {
    ///         if req.method == Method::Get {
    ///             Ok(Response::new(Version::Http11, 200))
    ///         } else {
    ///             Err(HttpError::NotFound)
    ///         }
    ///     });
    ///     Ok(())
    /// }
    /// ```
    fn handle(&self, req: Request) -> Result<Response, HttpError> {
        (self)(req)
    }
}

/// `Server`
///
/// The `Server` to bind to an address and handle incoming HTTP requests to it.
///
/// # Examples
/// ```rust, no_run
/// use foxpaw_habanero::*;
/// use foxpaw_habanero::err::{HttpError, ServerError};
///
/// fn start_server() -> Result<(), ServerError> {
///     let server = Server::bind("127.0.0.1:8080")?;
///     server.run(|req: Request| {
///         if req.method == Method::Get {
///             Ok(Response::new(Version::Http11, 200))
///         } else {
///             Err(HttpError::NotFound)
///         }
///     });
///     Ok(())
/// }
/// ```
pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Bind
    ///
    /// Bind to the provided address, ready to accept incoming request.
    ///
    /// # Errors
    /// This method will error with a [`ServerError`] if it is unable to bind
    /// to the underlying network `TcpListener`.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ServerError;
    ///
    /// fn start_server() -> Result<(), ServerError> {
    ///     let server = Server::bind("127.0.0.1:8080")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn bind(addr: impl ToSocketAddrs) -> Result<Self, ServerError> {
        let listener = TcpListener::bind(addr).map_err(|e| match e.kind() {
            io::ErrorKind::InvalidInput => ServerError::Invalid(e.to_string()),
            io::ErrorKind::PermissionDenied => ServerError::Bind(e.to_string()),
            io::ErrorKind::AddrInUse => ServerError::AddrInUse(e.to_string()),
            _ => ServerError::Other(e.to_string()),
        })?;
        Ok(Self { listener })
    }

    /// Run
    ///
    /// Run the server, using the provided handler to process each incoming
    /// [`Request`], and sending off an outgoing [`Response`].
    ///
    /// # Errors
    /// This method will error if there is an unrecoverable error, which will
    /// be triggered if the underlying listener fails to accept an incoming
    /// connection from a remote.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::{HttpError, ServerError};
    ///
    /// fn start_server() -> Result<(), ServerError> {
    ///     let server = Server::bind("127.0.0.1:8080")?;
    ///     server.run(|req: Request| {
    ///         if req.method == Method::Get {
    ///             Ok(Response::new(Version::Http11, 200))
    ///         } else {
    ///             Err(HttpError::NotFound)
    ///         }
    ///     });
    ///     Ok(())
    /// }
    /// ```
    pub fn run(&self, handler: impl Handler + 'static) -> Result<(), ServerError> {
        let handler = Arc::new(handler);

        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => {
                    let handler_clone = Arc::clone(&handler);
                    thread::spawn(move || {
                        let mut connection = Http1Server::new(Tcp::new(s));
                        loop {
                            let response = match connection.next_request() {
                                Ok(Some(req)) => handler_clone
                                    .handle(req)
                                    .unwrap_or_else(|e| Response::new(Version::Http11, e as u16)),
                                Ok(None) => break,
                                Err(_) => Response::new(Version::Http11, 500),
                            };

                            let _ = connection.send_response(response);
                            if !connection.is_persistent() {
                                break;
                            }
                        }
                    });
                }
                Err(e) => {
                    return Err(ServerError::Other(e.to_string()));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // impl Server

    #[test]
    fn server_bind_correct() {
        let actual = Server::bind("127.0.0.1:8080");
        assert!(actual.is_ok());
    }
}
