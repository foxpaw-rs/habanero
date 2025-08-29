//! HTTP requests.
//!
//! # Request format
//! As an HTTP request format has a number of optional fields, a `Request` is
//! initially built via a `Builder`. This allows for the addition of the
//! optional fields without requiring the `Request` to be mutable at any point.
//!
//! The HTTP request format requires a verb, path and version. Headers and the
//! request body are optional. For example both the following are valid HTTP
//! requests.
//! ```text
//! // Missing headers and a request body.
//! GET / HTTP/1.1
//!
//! // Providing headers and a request body.
//! POST /user HTTP/1.1
//! Content-Type: application/json
//! Content-Length: 35
//!
//! {
//!     "name": "John Doe",
//!     "age": 50
//! }
//! ```
//!
//! As the verb, path and version are all required, they must be initially
//! passed to the build method on `Request`. Headers and a request body can
//! then be added by calling the relevant methods on the `Builder`. The same
//! requests above would be constructed as so.
//!
//! ```
//! use habanero::request::*;
//! # fn main() {
//! // Missing headers and a request body.
//! Request::build(Verb::Get, "/", Version::Http1_1).create();
//!
//! // Providing headers and a request body.
//! Request::build(Verb::Post, "/user", Version::Http1_1)
//!     .header("Content-Type", "application/json")
//!     .header("Content-Length", "31")
//!     .body("{\"name\": \"John Doe\", \"age\": 50}")
//!     .create();
//! # }
//! ```
//!
//! # Examples
//!
//! Creating a `Request`.
//! ```rust
//! use habanero::request::*;
//!
//! # fn main() {
//! let request = Request::build(Verb::Get, "/", Version::Http1_1)
//!     .header("Content-Type", "application/json")
//!     .header("Content-Length", "7")
//!     .body("{ ... }")
//!     .create();
//! # }
//! ```

pub use crate::http::Version;
use core::fmt::{self, Debug, Display, Formatter};
use std::collections::BTreeMap;

/// HTTP Request Builder.
///
/// Utilises the builder pattern to fluently construct a `Request`. Each method
/// call invalidates the previous `Builder`, and it is intended to be chained
/// from initial construction all the way to the finalise, `create` method to
/// create the `Request`. If multiple `Requests` are required based off the
/// same set of information, the `Builder` should be cloned.
///
/// # Examples
/// ```rust
/// use habanero::{
///     Request,
///     request::{
///         Builder, Verb, Version
///     }
/// };
/// // Or use habanero::request::*;
///
/// let request = Request::build(Verb::Get, "/", Version::Http1_1)
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder<'a> {
    body: &'a str,
    headers: BTreeMap<&'a str, &'a str>,
    target: &'a str,
    verb: Verb,
    version: Version,
}

impl<'a> Builder<'a> {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Request::build` method to invoke the
    /// builder pattern and build up a `Request`.
    fn new(verb: Verb, target: &'a str, version: Version) -> Self {
        Builder {
            body: "",
            headers: BTreeMap::new(),
            target,
            verb,
            version,
        }
    }

    /// Set a `Request` body.
    ///
    /// Set a HTTP body on the `Request`. This will overwrite any previously
    /// set value for the request body.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb, Version
    ///     }
    /// };
    /// // Or use habanero::request::*;
    ///
    /// // Note: The final request body will be "{ ... }".
    /// let request = Request::build(Verb::Get, "/", Version::Http1_1)
    ///     .body("<html>...</html>")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn body(mut self, body: &'a str) -> Self {
        self.body = body;
        self
    }

    /// Create the built `Request`.
    ///
    /// Finalises the `Builder`, invalidating the current reference and
    /// creating the built `Request`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb, Version
    ///     }
    /// };
    /// // Or use habanero::request::*;
    ///
    /// let request = Request::build(Verb::Get, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request<'a> {
        Request::new(
            self.verb,
            self.target,
            self.version,
            self.headers,
            self.body,
        )
    }

    /// Set a `Request` header.
    ///
    /// Set a HTTP header on the `Request`. This will overwrite any previously
    /// set value for that header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb, Version
    ///     }
    /// };
    /// // Or use habanero::request::*;
    ///
    /// // Note: The final "Content-Type" header will be "application/html".
    /// let request = Request::build(Verb::Get, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .header("Content-Type", "application/html")
    ///     .create();
    /// ```
    #[must_use]
    pub fn header(mut self, key: &'a str, value: &'a str) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// A HTTP Request.
///
/// Stores information about the HTTP request, either recevied from a socket
/// (or `Server`), or built to be sent via a connection (or `Client`). `Requests`
/// are constructed using a builder pattern due to the nature of the different
/// information required to be contained within each `Request`.
///
/// # Examples
/// ```rust
/// use habanero::{
///     Request,
///     request::{
///         Builder, Verb, Version
///     }
/// };
/// // Or use habanero::request::*;
///
/// let request = Request::build(Verb::Get, "/", Version::Http1_1)
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Request<'a> {
    body: &'a str,
    headers: BTreeMap<&'a str, &'a str>,
    target: &'a str,
    verb: Verb,
    version: Version,
}

impl<'a> Request<'a> {
    /// Create a new `Request`.
    ///
    /// Creates a new request, invoked via the `Builder::create` method to
    /// finalise the construction of the `Request`.
    fn new(
        verb: Verb,
        target: &'a str,
        version: Version,
        headers: BTreeMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Self {
        Self {
            body,
            headers,
            target,
            verb,
            version,
        }
    }

    /// Build a new `Request`.
    ///
    /// Creates a `Builder`, used to construct the `Request`. `Requests` are
    /// created using the builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb, Version
    ///     }
    /// };
    /// // Or use habanero::request::*;
    ///
    /// let request = Request::build(Verb::Get, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(verb: Verb, target: &'a str, version: Version) -> Builder<'a> {
        Builder::new(verb, target, version)
    }
}

impl Display for Request<'_> {
    /// Format the `Request`.
    ///
    /// Formats the `Request` into an HTTP compatible request format, able to
    /// be sent to a server.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb, Version
    ///     }
    /// };
    /// // Or use habanero::request::*;
    ///
    /// let request = Request::build(Verb::Get, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// let string = request.to_string();
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}\n{}\n{}",
            self.verb,
            self.target,
            self.version,
            self.headers.iter().fold(String::new(), |fold, pair| {
                format!("{fold}{}: {}\n", pair.0, pair.1)
            }),
            self.body
        )
    }
}

/// The HTTP Verbs.
///
/// Representation of the supported HTTP verbs, or methods, which are sent via
/// the HTTP request.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Verb {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

impl Display for Verb {
    /// Format the `Verb`.
    ///
    /// Formats the `Verb` into what would be expected for an HTTP request.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::request::Verb;
    ///
    /// let verb = Verb::Connect;
    /// let string = verb.to_string();
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!("{self:?}").to_uppercase())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // impl Builder

    #[test]
    fn builder_new_success() {
        let expected = Builder {
            body: "",
            headers: BTreeMap::new(),
            target: "/",
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Verb::Get, "/", Version::Http1_1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let mut headers = BTreeMap::new();
        headers.insert("key", "value");

        let expected = Request {
            body: "body",
            headers: headers,
            target: "/",
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Verb::Get, "/", Version::Http1_1)
            .header("key", "value")
            .body("body")
            .create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_success() {
        let expected = "body";
        let actual = Builder::new(Verb::Get, "/", Version::Http1_1).body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = "body";
        let actual = Builder::new(Verb::Get, "/", Version::Http1_1)
            .body("not_body")
            .body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_header_success() {
        let mut expected = BTreeMap::new();
        expected.insert("key", "value");

        let actual = Builder::new(Verb::Get, "/", Version::Http1_1).header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_header_overwrite() {
        let mut expected = BTreeMap::new();
        expected.insert("key", "value");

        let actual = Builder::new(Verb::Get, "/", Version::Http1_1)
            .header("key", "not_value")
            .header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    // impl Request

    #[test]
    fn request_new_success() {
        let mut headers = BTreeMap::new();
        headers.insert("key", "value");

        let expected = Request {
            body: "body",
            headers: headers,
            target: "/",
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Request::build(Verb::Get, "/", Version::Http1_1)
            .header("key", "value")
            .body("body")
            .create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_builder_success() {
        let expected = Builder {
            body: "",
            headers: BTreeMap::new(),
            target: "/",
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Request::build(Verb::Get, "/", Version::Http1_1);
        assert_eq!(expected, actual);
    }

    // impl Display for Request

    #[test]
    fn request_fmt_success() {
        let expected = "\
        GET / HTTP/1.1\n\
        Content-Length: 16\n\
        Content-Type: application/json\n\n\
        {\"key\": \"value\"}";

        let actual = Request::build(Verb::Get, "/", Version::Http1_1)
            .header("Content-Type", "application/json")
            .header("Content-Length", "16")
            .body("{\"key\": \"value\"}")
            .create()
            .to_string();

        assert_eq!(expected, actual);
    }

    // impl Display for Verb

    #[test]
    fn verb_fmt_success() {
        let expected = "CONNECT";
        let actual = Verb::Connect.to_string();
        assert_eq!(expected, actual);
    }
}
