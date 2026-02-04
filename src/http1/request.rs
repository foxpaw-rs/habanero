//! Http1 Requests
//!
//! Todo(Paul): Module documentation.

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
/// // Todo(Paul): Add more as headers/body built out.
/// let request = Request::build(Verb::Post, "/")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
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
            verb,
            target: target.into(),
        }
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
    /// // Todo(Paul): Add more as headers/body built out.
    /// let request = Request::build(Verb::Post, "/")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request {
        Request::new(self.verb, self.target)
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
/// // Todo(Paul): Add more as headers/body built out.
/// let request = Request::build(Verb::Post, "/")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    target: String,
    verb: Verb,
}

impl Request {
    /// Create a new `Request`.
    ///
    /// Creates a new `Request` invoked via the `Builder::create` method to
    /// finalise the construction of the `Request`.
    fn new(verb: Verb, target: String) -> Self {
        Self { target, verb }
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
    /// // Todo(Paul): Add more as headers/body built out.
    /// let request = Request::build(Verb::Post, "/")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(verb: Verb, target: impl Into<String>) -> Builder {
        Builder::new(verb, target)
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
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Builder::new(Verb::Post, "/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Request {
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Builder::new(Verb::Post, "/").create();
        assert_eq!(expected, actual);
    }

    // impl Request

    #[test]
    fn request_new_success() {
        let expected = Request {
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Request::new(Verb::Post, String::from("/"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_create_success() {
        let expected = Builder {
            verb: Verb::Post,
            target: String::from("/"),
        };
        let actual = Request::build(Verb::Post, "/");
        assert_eq!(expected, actual);
    }
}
