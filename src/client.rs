//! Client
//!
//! Todo: Module documentation

use crate::common::{Request, Response};
use crate::connection::{Client as ClientConnection, Http1Client};
use crate::err::ClientError;
use crate::transport::Tcp;
use std::fmt::Display;
use std::net::TcpStream;

/// `Client`
///
/// Todo: Document / Tests
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Client;

impl Client {
    /// New
    ///
    /// Create a new `Client` to send HTTP requests.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Client;
    ///
    /// let client = Client::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Dispatch
    ///
    /// Dispatch a [`Request`] to the specified host and port. 
    ///
    /// # Errors
    /// This will error if the connection to the remote host fails, or if the
    /// [`Request`] serialization or [`Response`] parsing fails by the 
    /// [`Connection`].
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let req = Request::new(Method::Get, "/", Version::Http11);
    ///     let res = client.dispatch(req, "127.0.0.1", "8080")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn dispatch(
        &self,
        req: Request,
        host: impl AsRef<str> + Display,
        port: impl AsRef<str> + Display,
    ) -> Result<Response, ClientError> {
        let addr = format!("{}:{}", &host, &port);
        let stream = TcpStream::connect(addr)?;
        let transport = Tcp::new(stream);
        Ok(Http1Client::new(transport).send_request(req)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // impl Client

    #[test]
    fn client_new_correct() {
        let actual = Client::new();
        let expected = Client;
        assert_eq!(actual, expected);
    }
}
