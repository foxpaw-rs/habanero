//! HTTP requests.
//! Todo(Paul): Module documentation.

pub use crate::http::Version;
use std::collections::BTreeMap;

/// HTTP Request Builder.
///
/// Utilizes the builder pattern to fluently construct a `Request`. Each method
/// call invalidates the previous `Builder`, and it is intended to be chained
/// from initial construction all the way to the finalize, `create` method to
/// create the `Request`. If multiple `Requests` are required based off the
/// same set of information, the `Builder` should be cloned.
///
/// # Examples
///
/// ## Sending a typed body request
/// This will set the Content-Type and Content-Length headers automatically.
/// ```rust
/// use habanero::request::*;
/// // Or use habanero::{
/// //     Request,
/// //     request::{Builder, Verb, Version}
/// // };
///
/// let json = Request::build(Verb::Post, "/", Version::Http1_1)
///     .json("{ ... }")
///     .create();
///  
/// let url = Request::build(Verb::Post, "/", Version::Http1_1)
///     .url_encoded("name=MyName&email=test%40test.com")
///     .create();
///  
/// ```
///
/// ## Creating a basic body request
/// This will leave setting the Content-Type and Content-Length headers
/// appropriately to the implementer, if desired.
/// ```rust
/// use habanero::request::*;
/// // Or use habanero::{
/// //     Request,
/// //     request::{Builder, Verb, Version}
/// // };
///
/// let request = Request::build(Verb::Post, "/", Version::Http1_1)
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    body: String,
    headers: BTreeMap<String, String>,
    target: String,
    verb: Verb,
    version: Version,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Request::build` method to invoke the
    /// builder pattern and build up a `Request`.
    fn new(verb: Verb, target: impl Into<String>, version: Version) -> Self {
        Self {
            body: String::new(),
            headers: BTreeMap::new(),
            target: target.into(),
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
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// // Note: The final request body will be "{ ... }".
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
    ///     .body("Hello World")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
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
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request {
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
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// // Note: The final "Content-Type" header will be "text/html".
    /// let request = Request::build(Verb::Get, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .header("Content-Type", "text/html")
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
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
    ///     .json("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn json(mut self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();

        self.body = body;
        self.header("Content-Type", "application/json")
            .header("Content-Length", len)
    }

    /// Set a `Request` url encoded body.
    ///
    /// Set a url encoded HTTP body on the `Request`. This will overwrite any
    /// previously set value for the request body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
    ///     .url_encoded("name=MyName&email=test%40test.com")
    ///     .create();
    /// ```
    #[must_use]
    pub fn url_encoded(mut self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();

        self.body = body;
        self.header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", len)
    }
}

/// A HTTP Request.
///
/// Stores information about the HTTP request, either received from a socket
/// (or `Server`), or built to be sent via a connection (or `Client`). `Requests`
/// are constructed using a builder pattern due to the nature of the different
/// information required to be contained within each `Request`.
///
/// # Examples
/// ```rust
/// use habanero::request::*;
/// // Or use habanero::{
/// //     Request,
/// //     request::{Builder, Verb, Version}
/// // };
///
/// let request = Request::build(Verb::Post, "/", Version::Http1_1)
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    body: String,
    headers: BTreeMap<String, String>,
    target: String,
    verb: Verb,
    version: Version,
}

impl Request {
    /// Create a new `Request`.
    ///
    /// Creates a new request, invoked via the `Builder::create` method to
    /// finalise the construction of the `Request`.
    fn new(
        verb: Verb,
        target: String,
        version: Version,
        headers: BTreeMap<String, String>,
        body: String,
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
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(verb: Verb, target: impl Into<String>, version: Version) -> Builder {
        Builder::new(verb, target, version)
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
            body: "".to_string(),
            headers: BTreeMap::new(),
            target: "/".to_string(),
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Verb::Get, "/", Version::Http1_1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_success() {
        let expected = "body";
        let actual = Builder::new(Verb::Post, "/", Version::Http1_1).body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = "body";
        let actual = Builder::new(Verb::Post, "/", Version::Http1_1)
            .body("not_body")
            .body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_create_success() {
        let mut headers = BTreeMap::new();
        headers.insert("key".to_string(), "value".to_string());

        let expected = Request {
            body: "body".to_string(),
            headers: headers,
            target: "/".to_string(),
            verb: Verb::Post,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Verb::Post, "/", Version::Http1_1)
            .header("key", "value")
            .body("body")
            .create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_success() {
        let mut expected = BTreeMap::new();
        expected.insert("key".to_string(), "value".to_string());

        let actual = Builder::new(Verb::Get, "/", Version::Http1_1).header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_header_overwrite() {
        let mut expected = BTreeMap::new();
        expected.insert("key".to_string(), "value".to_string());

        let actual = Builder::new(Verb::Get, "/", Version::Http1_1)
            .header("key", "not_value")
            .header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_json_success() {
        let actual = Builder::new(Verb::Post, "/", Version::Http1_1).json("{ ... }");
        let expected = Builder::new(Verb::Post, "/", Version::Http1_1)
            .body("{ ... }")
            .header("Content-Type", "application/json")
            .header("Content-Length", "7");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_url_encoded_success() {
        let actual = Builder::new(Verb::Post, "/", Version::Http1_1)
            .url_encoded("name=MyName&email=test%40test.com");
        let expected = Builder::new(Verb::Post, "/", Version::Http1_1)
            .body("name=MyName&email=test%40test.com")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", "33");
        assert_eq!(expected, actual);
    }

    // impl Request

    #[test]
    fn request_new_success() {
        let mut headers = BTreeMap::new();
        headers.insert("key".to_string(), "value".to_string());

        let expected = Request {
            body: "body".to_string(),
            headers: headers.clone(),
            target: "/".to_string(),
            verb: Verb::Post,
            version: Version::Http1_1,
        };
        let actual = Request::new(
            Verb::Post,
            "/".to_string(),
            Version::Http1_1,
            headers,
            "body".to_string(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_build_success() {
        let expected = Builder {
            body: "".to_string(),
            headers: BTreeMap::new(),
            target: "/".to_string(),
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Request::build(Verb::Get, "/", Version::Http1_1);
        assert_eq!(expected, actual);
    }
}
