//! HTTP requests.
//! Todo(Paul): Module documentation.

pub use crate::http::Version;

/// HTTP Request Builder.
///
/// Utilizes the builder pattern to fluently construct a `Request`. Each method
/// call invalidates the previous `Builder`, and it is intended to be chained
/// from initial construction all the way to the finalize, `create` method to
/// create the `Request`. If multiple `Requests` are required based off the
/// same set of information, the `Builder` should be cloned.
///
/// # Examples
/// Todo(Paul): Examples once completed.
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
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
            target: target.into(),
            verb,
            version,
        }
    }

    /// Create the built `Request`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Request`.
    ///
    /// # Examples
    /// Todo(Paul): Add headers/body once implemented.
    /// ```rust
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request {
        Request::new(self.verb, self.target, self.version)
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
/// Todo(Paul): Examples once completed.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    target: String,
    verb: Verb,
    version: Version,
}

impl Request {
    /// Create a new `Request`.
    ///
    /// Creates a new request, invoked via the `Builder::create` method to
    /// finalise the construction of the `Request`.
    fn new(verb: Verb, target: String, version: Version) -> Self {
        Self {
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
    /// Todo(Paul): Add headers/body once implemented.
    /// ```rust
    /// use habanero::request::*;
    /// // Or use habanero::{
    /// //     Request,
    /// //     request::{Builder, Verb, Version}
    /// // };
    ///
    /// let request = Request::build(Verb::Post, "/", Version::Http1_1)
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
    // Todo(Paul): Add headers/body once implemented.
    fn builder_new_success() {
        let expected = Builder {
            target: "/".to_string(),
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Verb::Get, "/", Version::Http1_1);
        assert_eq!(expected, actual);
    }

    #[test]
    // Todo(Paul): Add headers/body once implemented.
    fn builder_create_success() {
        let expected = Request {
            target: "/".to_string(),
            verb: Verb::Post,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Verb::Post, "/", Version::Http1_1).create();
        assert_eq!(expected, actual);
    }

    // impl Request

    #[test]
    // Todo(Paul): Add headers/body once implemented.
    fn request_new_success() {
        let expected = Request {
            target: "/".to_string(),
            verb: Verb::Post,
            version: Version::Http1_1,
        };
        let actual = Request::new(Verb::Post, "/".to_string(), Version::Http1_1);
        assert_eq!(expected, actual);
    }

    #[test]
    // Todo(Paul): Add headers/body once implemented.
    fn request_build_success() {
        let expected = Builder {
            target: "/".to_string(),
            verb: Verb::Get,
            version: Version::Http1_1,
        };
        let actual = Request::build(Verb::Get, "/", Version::Http1_1);
        assert_eq!(expected, actual);
    }
}
