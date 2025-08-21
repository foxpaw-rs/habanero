//! Request
//! Todo(Paul): Module documentation

use std::collections::HashMap;

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
/// // Todo(Paul): Expand this
/// use habanero::{
///     Request,
///     request::{
///         Builder, Verb
///     }
/// };
///
/// let request = Request::build(Verb::Get, "/")
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder<'a> {
    body: &'a str,
    headers: HashMap<&'a str, &'a str>,
    target: &'a str,
    verb: Verb,
}

impl<'a> Builder<'a> {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Request::build` method to invoke the
    /// builder pattern and build up a `Request`.
    fn new(verb: Verb, target: &'a str) -> Self {
        Builder {
            body: "",
            headers: HashMap::new(),
            target,
            verb,
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
    ///         Builder, Verb
    ///     }
    /// };
    ///
    /// // Note: The final request body will be "{ ... }".
    /// let request = Request::build(Verb::Get, "/")
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
    /// // Todo(Paul): Expand this
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb
    ///     }
    /// };
    ///
    /// let request = Request::build(Verb::Get, "/")
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request<'a> {
        Request::new(self.verb, self.target, self.headers, self.body)
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
    ///         Builder, Verb
    ///     }
    /// };
    ///
    /// // Note: The final "Content-Type" header will be "application/html".
    /// let request = Request::build(Verb::Get, "/")
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
/// // Todo(Paul): Expand this
/// use habanero::{
///     Request,
///     request::{
///         Builder, Verb
///     }
/// };
///
/// let request = Request::build(Verb::Get, "/")
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Request<'a> {
    body: &'a str,
    headers: HashMap<&'a str, &'a str>,
    target: &'a str,
    verb: Verb,
}

impl<'a> Request<'a> {
    /// Create a new `Request`.
    ///
    /// Creates a new request, invoked via the `Builder::create` method to
    /// finalise the construction of the `Request`.
    fn new(verb: Verb, target: &'a str, headers: HashMap<&'a str, &'a str>, body: &'a str) -> Self {
        Self {
            body,
            headers,
            target,
            verb,
        }
    }

    /// Build a new `Request`.
    ///
    /// Creates a `Builder`, used to construct the `Request`. `Requests` are
    /// created using the builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// // Todo(Paul): Expand this
    /// use habanero::{
    ///     Request,
    ///     request::{
    ///         Builder, Verb
    ///     }
    /// };
    ///
    /// let request = Request::build(Verb::Get, "/")
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(verb: Verb, target: &'a str) -> Builder<'a> {
        Builder::new(verb, target)
    }
}

/// The HTTP Verbs
///
/// Representation of the avaiable HTTP verbs, or methods, which are sent via
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
            body: "",
            headers: HashMap::new(),
            target: "/",
            verb: Verb::Get,
        };
        let actual = Builder::new(Verb::Get, "/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let mut headers = HashMap::new();
        headers.insert("key", "value");

        let expected = Request {
            body: "body",
            headers: headers,
            target: "/",
            verb: Verb::Get,
        };
        let actual = Builder::new(Verb::Get, "/")
            .header("key", "value")
            .body("body")
            .create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_success() {
        let expected = "body";
        let actual = Builder::new(Verb::Get, "/").body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = "body";
        let actual = Builder::new(Verb::Get, "/").body("not_body").body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_header_success() {
        let mut expected = HashMap::new();
        expected.insert("key", "value");

        let actual = Builder::new(Verb::Get, "/").header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_header_overwrite() {
        let mut expected = HashMap::new();
        expected.insert("key", "value");

        let actual = Builder::new(Verb::Get, "/")
            .header("key", "not_value")
            .header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    // impl Request

    #[test]
    fn request_new_success() {
        let mut headers = HashMap::new();
        headers.insert("key", "value");

        let expected = Request {
            body: "body",
            headers: headers,
            target: "/",
            verb: Verb::Get,
        };
        let actual = Request::build(Verb::Get, "/")
            .header("key", "value")
            .body("body")
            .create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_builder_success() {
        let expected = Builder {
            body: "",
            headers: HashMap::new(),
            target: "/",
            verb: Verb::Get,
        };
        let actual = Request::build(Verb::Get, "/");
        assert_eq!(expected, actual);
    }
}
