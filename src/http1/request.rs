//! Http1 Requests
//!
//! Todo(Paul): Module documentation.

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
///     .header("Content-Type", "text/html")
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
    /// Create a new `Builder` via the `Request::build` method to invoked the
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
    /// Set a HTTP body on the `Request`. This will overwrite any previously
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

    /// Create the build `Request`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Request`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "text/html")
    ///     .body("Hello World")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request {
        Request::new(self.verb, self.target, self.headers, self.body)
    }

    /// Set a `Request` header.
    ///
    /// Set a HTTP header on the `Request`. This will overwrite any previously
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
///     .header("Content-Type", "text/html")
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
    /// finalise the construction of the `Request`.
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
    /// Creates a `Builder` used top construct the `Request`. `Requests` are
    /// created using a builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let request = Request::build(Verb::Post, "/")
    ///     .header("Content-Type", "text/html")
    ///     .body("Hello World")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(verb: Verb, target: impl Into<String>) -> Builder {
        Builder::new(verb, target)
    }

    /// Retrieve the specified `Request` header.
    ///
    /// Retrieve and immutable reference to the specified header stored in the
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
    /// Retrieve and immutable reference to the `Request` target.
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
    /// Retrieve and immutable reference to the `Request` verb.
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
        let actual = Builder::new(Verb::Get, "/").body("Hello World");
        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = String::from("Hello World");
        let actual = Builder::new(Verb::Get, "/")
            .body("Overwritten")
            .body("Hello World");
        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_create_success() {
        let expected = Request {
            body: String::new(),
            headers: BTreeMap::new(),
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Builder::new(Verb::Post, "/").create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_success() {
        let mut expected = BTreeMap::new();
        expected.insert(String::from("Key"), String::from("Hello World"));

        let actual = Builder::new(Verb::Get, "/").header("Key", "Hello World");
        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_header_overwrite() {
        let mut expected = BTreeMap::new();
        expected.insert(String::from("Key"), String::from("Hello World"));

        let actual = Builder::new(Verb::Get, "/")
            .header("Key", "Overwritten")
            .header("Key", "Hello World");
        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_json_success() {
        let actual = Builder::new(Verb::Post, "/").json("{ ... }");
        let expected = Builder::new(Verb::Post, "/")
            .body("{ ... }")
            .header("Content-Type", "application/json")
            .header("Content-Length", "7");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_url_encoded_success() {
        let actual = Builder::new(Verb::Post, "/").url_encoded("key=value");
        let expected = Builder::new(Verb::Post, "/")
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
        let expected = Some("text/html");

        let request = Request::build(Verb::Post, "/")
            .header("Content-Type", "text/html")
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
        let mut expected = BTreeMap::new();
        expected.insert("Content-Type".to_string(), "text/html".to_string());
        expected.insert("Content-Length".to_string(), "0".to_string());

        let request = Request::build(Verb::Post, "/")
            .header("Content-Type", "text/html")
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
}
