//! Client
//!
//! The client module is responsible for housing types that instigate outgoing
//! connections and requests to remote hosts.
//!
//! # Architecture
//! This module provides the [`Client`] struct to connect to remote
//! hosts.
//!
//! # Examples
//! ## Quick Requests
//! ```rust, no_run
//! use foxpaw_habanero::*;
//! use foxpaw_habanero::err::ClientError;
//!
//! fn send_request() -> Result<(), ClientError> {
//!     let client = Client::new();
//!
//!     // Send a request that does not use the body.
//!     let get_res = client.get(&"http://127.0.0.1:8080".parse()?)?;
//!
//!     // Send a request that does use the body.
//!     let post_res = client.post(&"http://127.0.0.1:8080".parse()?, b"Hello World!")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Dispatch a more complex request
//! ```rust, no_run
//! use foxpaw_habanero::*;
//! use foxpaw_habanero::err::ClientError;
//!
//! fn send_request() -> Result<(), ClientError> {
//!     let client = Client::new();
//!     let req = Request::new(Method::Post, "/", Version::Http11)
//!         .with_json(r#"{message: "Hello World"}"#);
//!     let res = client.dispatch(&"http://127.0.0.1:8080".parse()?, req)?;
//!     Ok(())
//! }
//! ```

use crate::common::{Method, Request, Response, Url, Version};
use crate::connection::{Client as ClientConnection, Http1Client};
use crate::err::ClientError;
use crate::transport::Tcp;
use std::net::TcpStream;

/// `Client`
///
/// Used to sent HTTP requests to a remote host. Provides a generic dispatch
/// method for sending a prebuilt [`Request`] and convenience methods for each
/// HTTP method for simpler requests.
///
/// # Examples
/// ## Quick Requests
/// ```rust, no_run
/// use foxpaw_habanero::*;
/// use foxpaw_habanero::err::ClientError;
///
/// fn send_request() -> Result<(), ClientError> {
///     let client = Client::new();
///
///     // Send a request that does not use the body.
///     let get_res = client.get(&"http://127.0.0.1:8080".parse()?)?;
///
///     // Send a request that does use the body.
///     let post_res = client.post(&"http://127.0.0.1:8080".parse()?, b"Hello World!")?;
///     Ok(())
/// }
/// ```
///
/// ## Dispatch a more complex request
/// ```rust, no_run
/// use foxpaw_habanero::*;
/// use foxpaw_habanero::err::ClientError;
///
/// fn send_request() -> Result<(), ClientError> {
///     let client = Client::new();
///     let req = Request::new(Method::Post, "/", Version::Http11)
///         .with_json(r#"{message: "Hello World"}"#);
///     let res = client.dispatch(&"http://127.0.0.1:8080".parse()?, req)?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

    /// Connect
    ///
    /// Send a connect request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.connect(&"http://127.0.0.1:8080".parse()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect(&self, url: &Url) -> Result<Response, ClientError> {
        let req = Request::new(Method::Connect, &url.path, Version::Http11);
        self.dispatch(url, req)
    }

    /// Delete
    ///
    /// Send a delete request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.delete(&"http://127.0.0.1:8080".parse()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn delete(&self, url: &Url) -> Result<Response, ClientError> {
        let req = Request::new(Method::Delete, &url.path, Version::Http11);
        self.dispatch(url, req)
    }

    /// Dispatch
    ///
    /// Dispatch a [`Request`] to the specified remote.
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
    ///     let res = client.dispatch(&"http://127.0.0.1:8080".parse()?, req)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn dispatch(&self, url: &Url, mut req: Request) -> Result<Response, ClientError> {
        let addr = format!("{}:{}", url.host, url.port);
        if req.headers.find("Host").is_none() {
            req = req.with_header("Host", &addr);
        }
        let stream = TcpStream::connect(addr)?;
        let transport = Tcp::new(stream);
        Ok(Http1Client::new(transport).send_request(req)?)
    }

    /// Get
    ///
    /// Send a get request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.get(&"http://127.0.0.1:8080".parse()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get(&self, url: &Url) -> Result<Response, ClientError> {
        let req = Request::new(Method::Get, &url.path, Version::Http11);
        self.dispatch(url, req)
    }

    /// Head
    ///
    /// Send a head request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.head(&"http://127.0.0.1:8080".parse()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn head(&self, url: &Url) -> Result<Response, ClientError> {
        let req = Request::new(Method::Head, &url.path, Version::Http11);
        self.dispatch(url, req)
    }

    /// Options
    ///
    /// Send an options request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.options(&"http://127.0.0.1:8080".parse()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn options(&self, url: &Url) -> Result<Response, ClientError> {
        let req = Request::new(Method::Options, &url.path, Version::Http11);
        self.dispatch(url, req)
    }

    /// Patch
    ///
    /// Send a patch request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.patch(&"http://127.0.0.1:8080".parse()?, b"Hello World")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn patch(&self, url: &Url, body: &[u8]) -> Result<Response, ClientError> {
        let req = Request::new(Method::Patch, &url.path, Version::Http11)
            .with_body(body);
        self.dispatch(url, req)
    }

    /// Post
    ///
    /// Send a post request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.post(&"http://127.0.0.1:8080".parse()?, b"Hello World")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn post(&self, url: &Url, body: &[u8]) -> Result<Response, ClientError> {
        let req = Request::new(Method::Post, &url.path, Version::Http11)
            .with_body(body);
        self.dispatch(url, req)
    }

    /// Put
    ///
    /// Send a put request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.put(&"http://127.0.0.1:8080".parse()?, b"Hello World")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn put(&self, url: &Url, body: &[u8]) -> Result<Response, ClientError> {
        let req = Request::new(Method::Put, &url.path, Version::Http11)
            .with_body(body);
        self.dispatch(url, req)
    }

    /// Query
    ///
    /// Send a query request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.query(&"http://127.0.0.1:8080".parse()?, b"Hello World")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn query(&self, url: &Url, body: &[u8]) -> Result<Response, ClientError> {
        let req = Request::new(Method::Query, &url.path, Version::Http11)
            .with_body(body);
        self.dispatch(url, req)
    }

    /// Trace
    ///
    /// Send a trace request to the desired remote.
    ///
    /// # Errors
    /// This will error if [`Client::dispatch`] to the remote host fails.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ClientError;
    ///
    /// fn send_request() -> Result<(), ClientError> {
    ///     let client = Client::new();
    ///     let res = client.trace(&"http://127.0.0.1:8080".parse()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn trace(&self, url: &Url) -> Result<Response, ClientError> {
        let req = Request::new(Method::Trace, &url.path, Version::Http11);
        self.dispatch(url, req)
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
