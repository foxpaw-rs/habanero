//! Server
//!
//! Todo: Module documentation

use crate::common::{Request, Response};
use crate::err::{HttpError, ServerError};
use std::io;
use std::net::{TcpListener, ToSocketAddrs};

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
    /// ```rust
    /// // Todo: Use a function as a server handler.
    /// ```
    fn handle(&self, req: Request) -> Result<Response, HttpError> {
        (self)(req)
    }
}

/// `Server`
///
/// Todo: Document
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_bind_correct() {
        let actual = Server::bind("127.0.0.1:8080");
        assert!(actual.is_ok());
    }
}
