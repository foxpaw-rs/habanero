//! Transport
//!
//! Todo: Module documentation

use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr};
use std::time::Duration;

/// Transport
///
/// Abstracts away reading, writing and other socket related activity so that
/// protocols can run over raw TCP, TLS or a mocked type for testing.
pub trait Transport: Read + Write + Send {
    /// Set Read Timeout
    ///
    /// Set the read timeout for this `Transport` type by the specified
    /// duration.
    ///
    /// # Errors
    /// Will return an `std::io::Error` if the read timeout is unable to be
    /// set.
    fn set_read_timeout(&mut self, duration: Option<Duration>) -> io::Result<()>;

    /// Set Write Timeout
    ///
    /// Set the write timeout for this `Transport` type by the specified
    /// duration.
    ///
    /// # Errors
    /// Will return an `std::io::Error` if the write timeout is unable to be
    /// set.
    fn set_write_timeout(&mut self, duration: Option<Duration>) -> io::Result<()>;

    /// Peer Addr
    ///
    /// Obtain the peer address the `Transport` is connected to.
    ///
    /// # Errors
    /// Will return an `std::io::Error` unable to obtain the peer address.
    fn peer_addr(&self) -> io::Result<SocketAddr>;

    /// Shutdown
    ///
    /// Shut down this `Transport` using the specified method.
    ///
    /// # Errors
    /// Will return an `std::io::Error` if the `Transport` cannot be shutdown
    /// as requested.
    fn shutdown(&mut self, how: Shutdown) -> io::Result<()>;
}
