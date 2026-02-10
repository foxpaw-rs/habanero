//! HTTP/1.1 Connection.
//!
//! Todo(Paul): Module documentation.

use std::net::{TcpStream, ToSocketAddrs};

/// An HTTP/1.1 Connction.
///
/// A single connection between this application and a remote host. Can be used
/// to both send and receive data.
#[derive(Debug)]
pub(crate) struct Connection {
    remote: TcpStream,
}

impl Connection {
    /// Create a new `Connection`.
    ///
    /// Creates a new `Connection`, signalling a single connection to a remote
    /// host.
    pub(crate) fn new(remote: impl ToSocketAddrs) -> Result<Self, u8> {
        Ok(Self {
            remote: TcpStream::connect(remote).map_err(|_| 0)?,
        })
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

    // impl Connection

    #[test]
    fn connection_new_success() {
        REMOTE.get_or_init(setup);
        let connection = Connection::new("localhost:7878");
        assert!(connection.is_ok());
    }

    #[test]
    fn connection_new_error() {
        let connection = Connection::new("localhost:8080");
        assert!(connection.is_err());
    }
}
