//! Todo(Paul): Module documentation.

use std::net::ToSocketAddrs;

/// HTTP Client Builder.
///
/// Utilizes the builder pattern to fluently construct a `Client`. Each
/// method call invalidates the previous `Builder`, and it is intended to be
/// chained from initial construction all the way to the finalize, `create`
/// method to create the `Client`. If multiple `Clients` are required based
/// off the same set of information, the `Builder` should be cloned.
///
/// # Examples
/// ```rust
/// use habanero::client::*;
/// // Or use habanero::{
/// //      Client,
/// //      client::Builder
/// //  };
//
/// let response = Client::build("foxpaw.rs")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder<A>
where
    A: ToSocketAddrs,
{
    addr: A,
}

impl<A> Builder<A>
where
    A: ToSocketAddrs,
{
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Client::build` method to invoke the
    /// builder pattern and build up a `Client`.
    fn new(addr: A) -> Self {
        Self { addr }
    }

    /// Create the built `Client`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Client`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::client::*;
    /// // Or use habanero::{
    /// //      Client,
    /// //      client::Builder
    /// //  };
    ///
    /// let client = Client::build("foxpaw.rs")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Client<A> {
        Client::new(self.addr)
    }
}

/// A HTTP Client.
///
/// Stores information about the HTTP `Client`, used to send `Requests` to
/// remote servers and receive and return `Responses`. `Clients` are desiged
/// to be reused when connecting to the same remote host with the same
/// configuration.
///
/// # Examples
/// ```rust
/// use habanero::client::*;
/// // Or use habanero::{
/// //      Client,
/// //      client::Builder
/// //  };
///
/// let client = Client::build("foxpaw.rs")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Client<A>
where
    A: ToSocketAddrs,
{
    addr: A,
}

impl<A> Client<A>
where
    A: ToSocketAddrs,
{
    /// Create a new `Client`.
    ///
    /// Creates a new client, invoked via the `Builder::create` method to
    /// finalise the construction of the `Client`.
    fn new(addr: A) -> Self {
        Self { addr }
    }

    /// Build a new `Client`.
    ///
    /// Creates a `Builder`, used to construct the `Client`. `Clients` are
    /// created using the builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::client::*;
    /// // Or use habanero::{
    /// //      Client,
    /// //      client::Builder
    /// //  };
    ///
    /// let client = Client::build("foxpaw.rs")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build(addr: A) -> Builder<A> {
        Builder::new(addr)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // impl Builder

    #[test]
    fn builder_new_success() {
        let expected = Builder { addr: "foxpaw.rs" };
        let actual = Builder::new("foxpaw.rs");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Client { addr: "foxpaw.rs" };
        let actual = Builder::new("foxpaw.rs").create();
        assert_eq!(expected, actual);
    }

    // impl Client

    #[test]
    fn request_new_success() {
        let expected = Client { addr: "foxpaw.rs" };
        let actual = Client::new("foxpaw.rs");
        assert_eq!(expected, actual);
    }

    #[test]
    fn request_build_success() {
        let expected = Builder { addr: "foxpaw.rs" };
        let actual = Client::build("foxpaw.rs");
        assert_eq!(expected, actual);
    }
}
