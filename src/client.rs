//! HTTP Client.
//!
//! Todo(Paul): Module documentation.

use crate::http1::Connection;
use std::net::ToSocketAddrs;

/// Client Builder.
///
/// Utilizes the builder pattern to fluently construct a `Client`. Each method
/// call invalidates the previous `Builder`, and it is intended to be chained
/// from the initial construction all the way to the finalizing `create` method
/// to creates the `Client`.
///
/// # Examples
/// ```rust
/// use habanero::Client;
///
/// let client = Client::build("localhost:8080")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder<A>
where
    A: ToSocketAddrs,
{
    remote: A,
}

impl<A> Builder<A>
where
    A: ToSocketAddrs,
{
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Client::build` method to invoke the
    /// builder pattern and build up a `Client`.
    fn new(remote: A) -> Self {
        Self { remote }
    }

    /// Create the built `Client`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Client`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::Client;
    ///
    /// let client = Client::build("localhost:8080")
    ///     .create();
    /// ```
    pub fn create(self) -> Result<Client, u8> {
        Client::new(self.remote)
    }
}

/// An HTTP Client.
///
/// Connects to a remote peer and sends HTTP `Requests`, receiving and
/// returning `Responses`. `Clients` are designed to be reused when
/// connecting to the same remote host with the same configuration.
///
/// # Examples
/// ```rust
/// use habanero::Client;
///
/// let client = Client::build("localhost:8080")
///     .create();
/// ```
#[derive(Debug)]
pub struct Client {
    remote: Connection,
}

impl Client {
    /// Create a new `Client`.
    ///
    /// Creates a new `Client`, invoked via the `Builder::create` method to
    /// finalize the construction of the `Client`
    fn new(remote: impl ToSocketAddrs) -> Result<Self, u8> {
        Ok(Self {
            remote: Connection::new(remote)?,
        })
    }

    /// Build a new `Client`.
    ///
    /// Creates a `Builder` used to construct the `Client`. `Clients` are
    /// created using a builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::Client;
    ///
    /// let client = Client::build("localhost:8080")
    ///     .create();
    /// ```
    #[must_use]
    pub fn build<A>(remote: A) -> Builder<A>
    where
        A: ToSocketAddrs,
    {
        Builder::new(remote)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::net::TcpListener;
    use std::sync::OnceLock;

    static REMOTE: OnceLock<TcpListener> = OnceLock::new();
    fn setup() -> TcpListener {
        TcpListener::bind("localhost:7878").unwrap()
    }

    // impl Builder

    #[test]
    fn builder_new_success() {
        let expected = Builder {
            remote: "localhost:7878",
        };
        let actual = Builder::new("localhost:7878");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        REMOTE.get_or_init(setup);
        let client = Builder::new("localhost:7878").create();
        assert!(client.is_ok());
    }

    #[test]
    fn builder_create_error() {
        let client = Builder::new("localhost:8080").create();
        assert!(client.is_err());
    }

    // impl Client

    #[test]
    fn client_new_success() {
        REMOTE.get_or_init(setup);
        let client = Client::new("localhost:7878");
        assert!(client.is_ok());
    }

    #[test]
    fn client_new_error() {
        let client = Client::new("localhost:8080");
        assert!(client.is_err());
    }

    #[test]
    fn client_build_success() {
        let expected = Builder {
            remote: "localhost:7878",
        };
        let actual = Client::build("localhost:7878");
        assert_eq!(expected, actual);
    }
}
