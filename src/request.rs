//! Request
//! Todo(Paul): Module documentation

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
///     .create();
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Builder<'a> {
    target: &'a str,
    verb: Verb,
}

impl<'a> Builder<'a> {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Request::build` method to invoke the
    /// builder pattern and build up a `Request`.
    fn new(verb: Verb, target: &'a str) -> Self {
        Builder { target, verb }
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
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Request<'a> {
        Request::new(self.verb, self.target)
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
///     .create();
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Request<'a> {
    target: &'a str,
    verb: Verb,
}

impl<'a> Request<'a> {
    /// Create a new `Request`.
    ///
    /// Creates a new request, invoked via the `Builder::create` method to
    /// finalise the construction of the `Request`.
    fn new(verb: Verb, target: &'a str) -> Self {
        Self { target, verb }
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
            target: "/",
            verb: Verb::Get,
        };
        let actual = Builder::new(Verb::Get, "/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Request {
            target: "/",
            verb: Verb::Get,
        };
        let actual = Builder::new(Verb::Get, "/").create();
        assert_eq!(expected, actual);
    }

    // impl Request

    #[test]
    fn request_new_success() {
        let expected = Request {
            target: "/",
            verb: Verb::Get,
        };
        let actual = Request::build(Verb::Get, "/").create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_builder_success() {
        let expected = Builder {
            target: "/",
            verb: Verb::Get,
        };
        let actual = Request::build(Verb::Get, "/");
        assert_eq!(expected, actual);
    }
}
