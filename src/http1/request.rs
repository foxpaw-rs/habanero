//! HTTP/1.1 requests.
//!
//! # Request format
//! As an HTTP/1.1 request format has a number of optional fields, a `Request`
//! is initially built via a `Builder`. This allows for the addition of the
//! optional fields without requiring the `Request` to be mutable at any point.
//!
//! The HTTP/1.1 request format requires a verb and target. Headers and the
//! request body are optional. For example both the following are valid HTTP
//! requests.
//! ```text
//! // Without headers and a request body.
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
//! As the verb and target are all required, they must be initially passed to
//! the build method on `Request`. Headers and a request body can then be added
//! by calling the relevant methods on the `Builder`. The same requests above
//! would be constructed as so.
//!
//! ```rust
//! use habanero::http1::*;
//!
//! // Without headers and a request body.
//! Request::build(Verb::Get, "/").create();
//!
//! // Providing headers and a request body.
//! Request::build(Verb::Post, "/user")
//!     .header("Content-Type", "application/json")
//!     .header("Content-Length", "31")
//!     .body("{\"name\": \"John Doe\", \"age\": 50}")
//!     .create();
//! ```
//!
//! The building process also provides shortcut methods for setting the request
//! to contain json or form url-encoded data, by also setting the
//! `Content-Type` and `Content-Length` header fields.
//!
//! ```rust
//! use habanero::http1::*;
//!
//! let json = Request::build(Verb::Post, "/")
//!     .json("{ ... }")
//!     .create();
//!
//! let url = Request::build(Verb::Post, "/")
//!     .url_encoded("key=value")
//!     .create();
//!
//! ```
//!
//! # Accessing Request data
//! To access the internal Request data once constructed, access methods are
//! provided on the Request type itself.
//!
//! ```rust
//! use habanero::http1::*;
//!
//! let request = Request::build(Verb::Get, "/")
//!     .create();
//! let verb = request.verb();
//! ```

use core::fmt::{self, Debug, Display, Formatter};
use std::collections::BTreeMap;

/// HTTP Request Builder.
///
/// Utilizes the builder pattern to fluently construct a `Request`. Each method
/// call invalidates the previous `Builder`, and it is intended to be chained
/// from the initial construction all the way to the finalizing `create` method
/// to create the `Request`. If multiple `Requests` are required based off the
/// same set of information the `Builder` should be cloned.
///
/// # Examples
/// ```rust
/// use habanero::http1::*;
///
/// let request = Request::build(Verb::Post, "/")
///     .header("Content-Type", "text/plain")
///     .body("Hello World")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    body: String,
    headers: BTreeMap<String, String>,
    target: String,
    verb: Verb,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Request::build` method to invoke the
    /// builder pattern and build up a `Request`.
    fn new(verb: Verb, target: impl Into<String>) -> Self {
        Self {
            body: String::new(),
            headers: BTreeMap::new(),
            verb,
            target: target.into(),
        }
    }

    /// Set a `Request` body.
    ///
    /// Set an HTTP body on the `Request`. This will overwrite any previously
    /// set value for the body.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .body("Hello World!")
    ///     .create();
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// Create the built `Request`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Request`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "text/plain")
    ///     .body("Hello World")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request {
        Request::new(self.verb, self.target, self.headers, self.body)
    }

    /// Set a `Request` header.
    ///
    /// Set an HTTP header on the `Request`. This will overwrite any previously
    /// set value for that header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Get, "/")
    ///     .header("Content-Type", "application/json")
    ///     .create();
    /// ```
    #[must_use]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set a `Request` JSON body.
    ///
    /// Set a JSON HTTP body on the `Request`. This will overwrite any
    /// previously set value for the request body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .json("{...}")
    ///     .create();
    /// ```
    #[must_use]
    pub fn json(self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len();

        self.body(body)
            .header("Content-Type", "application/json")
            .header("Content-Length", len.to_string())
    }

    /// Set a `Request` url encoded body.
    ///
    /// Set a url encoded HTTP body on the `Request`. This will overwrite any
    /// previously set value for the body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .url_encoded("key=value")
    ///     .create();
    /// ```
    #[must_use]
    pub fn url_encoded(self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len();

        self.body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", len.to_string())
    }
}

/// A HTTP Request.
///
/// Stores information about the HTTP request, either received from a socket
/// (or `Server`), or built to be sent via a connection (or `Client`).
/// `Request`s are constructed using a builder pattern due to the nature of the
/// different information required to be contained within each `Request`.
///
/// # Examples
/// ```rust
/// use habanero::http1::*;
///
/// let request = Request::build(Verb::Post, "/")
///     .header("Content-Type", "text/plain")
///     .body("Hello World")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    body: String,
    headers: BTreeMap<String, String>,
    target: String,
    verb: Verb,
}

impl Request {
    /// Create a new `Request`.
    ///
    /// Creates a new `Request` invoked via the `Builder::create` method to
    /// finalize the construction of the `Request`.
    fn new(verb: Verb, target: String, headers: BTreeMap<String, String>, body: String) -> Self {
        Self {
            body,
            headers,
            target,
            verb,
        }
    }

    /// Retrieve the `Request` body.
    ///
    /// Retrieve an immutable reference to the body stored in this `Request`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .body("Hello World")
    ///     .create();
    /// let body = request.body();
    /// ```
    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Build a new `Request`
    ///
    /// Creates a `Builder` used to construct the `Request`. `Requests` are
    /// created using a builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "text/plain")
    ///     .body("Hello World")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(verb: Verb, target: impl Into<String>) -> Builder {
        Builder::new(verb, target)
    }

    /// Retrieve the specified `Request` header.
    ///
    /// Retrieve an immutable reference to the specified header stored in the
    /// `Request`. Will return None if the requested header is not set.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "application/json")
    ///     .create();
    /// let header = request.header("Content-Type");
    /// ```
    #[must_use]
    pub fn header(&self, key: impl Into<String>) -> Option<&str> {
        self.headers.get(&key.into()).map(String::as_str)
    }

    /// Retrieve the `Request` headers.
    ///
    /// Retrieve an immutable reference to all the headers stored in the
    /// `Request`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "application/json")
    ///     .create();
    /// let headers = request.headers();
    /// ```
    #[must_use]
    pub fn headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }

    /// Retrieve the `Request` target.
    ///
    /// Retrieve an immutable reference to the `Request` target.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .create();
    /// let target = request.target();
    /// ```
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Retrieve the `Request` verb.
    ///
    /// Retrieve an immutable reference to the `Request` verb.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .create();
    /// let verb = request.verb();
    /// ```
    #[must_use]
    pub fn verb(&self) -> &Verb {
        &self.verb
    }
}

impl Display for Request {
    /// Format the `Request`.
    ///
    /// Formats the `Request` into an HTTP compatible request format.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "text/plain")
    ///     .body("Hello World")
    ///     .create();
    /// let string = request.to_string();
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} HTTP/1.1\n{}\n{}",
            self.verb,
            self.target,
            self.headers.iter().fold(String::new(), |fold, pair| {
                format!("{fold}{}: {}\n", pair.0, pair.1)
            }),
            self.body
        )
    }
}

/// The HTTP Verbs.
///
/// Representation of the supported HTTP verbs, or methods.
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
    /// use habanero::http1::Verb;
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
            body: String::new(),
            headers: BTreeMap::new(),
            target: String::from("/"),
            verb: Verb::Post,
        };
        let actual = Builder::new(Verb::Post, "/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_success() {
        let expected = String::from("Hello World");
        let actual = Builder::new(Verb::Get, "/").body("Hello World").body;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = String::from("Hello World");
        let actual = Builder::new(Verb::Get, "/")
            .body("Overwritten")
            .body("Hello World")
            .body;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Request {
            body: String::from("Hello World"),
            headers: BTreeMap::from([(String::from("Content-Type"), String::from("text/plain"))]),
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Builder::new(Verb::Post, "/")
            .header("Content-Type", "text/plain")
            .body("Hello World")
            .create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_success() {
        let expected = BTreeMap::from([(String::from("Key"), String::from("Hello World"))]);
        let actual = Builder::new(Verb::Get, "/")
            .header("Key", "Hello World")
            .headers;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_overwrite() {
        let expected = BTreeMap::from([(String::from("Key"), String::from("Hello World"))]);
        let actual = Builder::new(Verb::Get, "/")
            .header("Key", "Overwritten")
            .header("Key", "Hello World")
            .headers;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_json_success() {
        let expected = Builder::new(Verb::Post, "/").json("{ ... }");
        let actual = Builder::new(Verb::Post, "/")
            .body("{ ... }")
            .header("Content-Type", "application/json")
            .header("Content-Length", "7");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_url_encoded_success() {
        let expected = Builder::new(Verb::Post, "/").url_encoded("key=value");
        let actual = Builder::new(Verb::Post, "/")
            .body("key=value")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", "9");
        assert_eq!(expected, actual);
    }

    // impl Request

    #[test]
    fn request_new_success() {
        let expected = Request {
            body: String::new(),
            headers: BTreeMap::new(),
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Request::new(
            Verb::Post,
            String::from("/"),
            BTreeMap::new(),
            String::new(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_body_success() {
        let expected = "Hello World";
        let request = Request::build(Verb::Post, "/").body("Hello World").create();
        let actual = request.body();
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_build_success() {
        let expected = Builder {
            body: String::new(),
            headers: BTreeMap::new(),
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Request::build(Verb::Post, "/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_header_success() {
        let expected = Some("text/plain");
        let request = Request::build(Verb::Post, "/")
            .header("Content-Type", "text/plain")
            .create();
        let actual = request.header("Content-Type");
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_header_missing() {
        let expected = None;
        let request = Request::build(Verb::Get, "/").create();
        let actual = request.header("Content-Type");
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_headers_success() {
        let expected = BTreeMap::from([
            (String::from("Content-Type"), String::from("text/plain")),
            (String::from("Content-Length"), String::from("0")),
        ]);
        let request = Request::build(Verb::Post, "/")
            .header("Content-Type", "text/plain")
            .header("Content-Length", "0")
            .create();
        let actual = request.headers();

        assert_eq!(expected, *actual);
    }

    #[test]
    fn request_target_success() {
        let expected = "/";
        let request = Request::build(Verb::Get, "/").create();
        let actual = request.target();
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_verb_success() {
        let expected = Verb::Get;
        let request = Request::build(Verb::Get, "/").create();
        let actual = request.verb();
        assert_eq!(expected, *actual);
    }

    // impl Display for Request

    #[test]
    fn request_fmt_success() {
        let expected = "\
        POST / HTTP/1.1\n\
        Content-Length: 11\n\
        Content-Type: text/plain\n\
        \n\
        Hello World";
        let actual = Request::build(Verb::Post, "/")
            .header("Content-Type", "text/plain")
            .header("Content-Length", "11")
            .body("Hello World")
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
