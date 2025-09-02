//! Todo(Paul): Module documentation.

pub use crate::http::Version;

/// HTTP Response Builder.
///
/// Utilises the builder pattern to fluently construct a `Response`. Each
/// method call invalidates the previous `Builder`, and it is intended to be
/// chained from initial construction all the way to the finalise, `create`
/// method to create the `Response`. If multiple `Responses` are required based
/// off the same set of information, the `Builder` should be cloned.
///
/// # Examples
/// ```rust
/// use habanero::{
///     Response,
///     response::{
///         Builder, Version
///     }
/// };
/// // Or use habanero::response::*;
///
/// // Todo(Paul): Update this as filled out.
/// let response = Response::build(Version::Http1_1)
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    version: Version,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Response::build` method to invoke the
    /// builder pattern and build up a `Response`.
    fn new(version: Version) -> Self {
        Builder { version }
    }

    /// Create the built `Response`.
    ///
    /// Finalises the `Builder`, invalidating the current reference and
    /// creating the built `Response`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Response,
    ///     response::{
    ///         Builder, Version
    ///     }
    /// };
    /// // Or use habanero::response::*;
    ///
    /// // Todo(Paul): Update this as filled out.
    /// let response = Response::build(Version::Http1_1)
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Response {
        Response::new(self.version)
    }
}

/// A HTTP Response.
///
/// Stores information about the HTTP response, either recevied from a
/// connection (or `Client`), or built to be sent via a socket (or `Server`).
/// `Responses` are constructed using a builder pattern due to the nature of
///  the different information required to be contained within each `Response`.
///
/// # Examples
/// ```rust
/// use habanero::{
///     Response,
///     response::{
///         Builder, Version
///     }
/// };
/// // Or use habanero::response::*;
///
/// // Todo(Paul): Update this as filled out.
/// let response = Response::build(Version::Http1_1)
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    version: Version,
}

impl Response {
    /// Create a new `Response`.
    ///
    /// Creates a new response, invoked via the `Builder::create` method to
    /// finalise the construction of the `Response`.
    fn new(version: Version) -> Self {
        Self { version }
    }

    /// Build a new `Response`.
    ///
    /// Creates a `Builder`, used to construct the `Response`. `Responses` are
    /// created using the builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::{
    ///     Response,
    ///     response::{
    ///         Builder, Version
    ///     }
    /// };
    /// // Or use habanero::response::*;
    ///
    /// let builder = Response::build(Version::Http1_1);
    /// ```
    #[must_use]
    pub fn build(version: Version) -> Builder {
        Builder::new(version)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // impl Builder

    #[test]
    fn builder_new_success() {
        let expected = Builder {
            version: Version::Http1_1,
        };
        let actual = Builder::new(Version::Http1_1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Response {
            version: Version::Http1_1,
        };

        let actual = Builder::new(Version::Http1_1).create();
        assert_eq!(expected, actual);
    }

    // impl Response

    #[test]
    fn response_new_success() {
        let expected = Response {
            version: Version::Http1_1,
        };
        let actual = Response::build(Version::Http1_1).create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_build_success() {
        let expected = Builder {
            version: Version::Http1_1,
        };
        let actual = Response::build(Version::Http1_1);
        assert_eq!(expected, actual);
    }
}
