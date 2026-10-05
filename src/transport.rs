//! Transport
//!
//! This module houses the transport data types. These types wrap and handle
//! any connection to an external source (e.g. TCP, TLS or local connection for
//! testing).
//!
//! # Architecture
//! This module primarily provides the [`Transport`] trait to define the
//! requirements of any other type defined in this module.
//!
//! It also exposes implementing types, being:
//! * [`Tcp`], a connection over TCP.
//!
//! # Examples
//! ## Using a [`Tcp`]
//! ```rust, no_run
//! use foxpaw_habanero::transport::*;
//! use std::net::TcpStream;
//!
//! pub fn tcp_transport() -> std::io::Result<()> {
//!     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
//!
//!     // Read and write data over Tcp
//!     let mut read_buffer = [0_u8; 100];
//!     let num_read_bytes = transport.read(&mut read_buffer)?;
//!     Ok(())
//! }

use std::io;
pub use std::io::{Read, Write};
pub use std::net::{Shutdown, SocketAddr, TcpStream};
pub use std::time::Duration;

/// `Tcp`
///
/// Transport implementation over a TCP connection. This implementation only
/// supports plain text transportation of data and is unencrypted (e.g. http
/// connections).
///
/// # Examples
/// ```rust, no_run
/// use foxpaw_habanero::transport::*;
/// use std::net::TcpStream;
///
/// pub fn tcp_transport() -> std::io::Result<()> {
///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
///
///     // Read and write data over Tcp
///     let mut read_buffer = [0_u8; 100];
///     let num_read_bytes = transport.read(&mut read_buffer)?;
///
///     let mut write_buffer = "Hello World".as_bytes();
///     let num_written_bytes = transport.write(&mut read_buffer)?;
///
///     // Set timeouts and view the peer address
///     let peer = transport.peer_addr()?;
///
///     // Shutdown the Tcp once finished
///     transport.shutdown(Shutdown::Both)?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Tcp {
    inner: TcpStream,
}

impl Tcp {
    /// New
    ///
    /// Create a new `Tcp` instance over the provided `TcpStream`.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn create_tcp_transport() -> std::io::Result<Tcp> {
    ///     let transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     Ok(transport)
    /// }
    /// ```
    #[must_use]
    pub fn new(inner: TcpStream) -> Self {
        Self { inner }
    }
}

impl Read for Tcp {
    /// Read
    ///
    /// Read bytes into the provided buffer, returning the number of bytes read
    /// on success.
    ///
    /// # Errors
    /// This method will return an `std::io::Error` in one of two
    /// circumstances:
    /// * The buffer has a length of 0; or
    /// * The reader has reached end of file and is unable to return further
    ///   bytes. Note that this does not mean the reader will never be able to
    ///   receive further data.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn read_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     let mut buffer = [0_u8; 100];
    ///     let count = transport.read(&mut buffer)?;
    ///     Ok(())
    /// }
    /// ```
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Transport for Tcp {
    /// Set Read Timeout
    ///
    /// Set the read timeout for this `Tcp`  by the specified duration. If None
    /// is passed as the duration the Transport will block indefinitely.
    ///
    /// # Errors
    /// Errors if a zero `Duration` is passed to this operation.
    ///
    /// # Examples
    ///```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn read_timeout_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     transport.set_read_timeout(Some(Duration::new(10, 0)))?;
    ///     Ok(())
    /// }
    /// ```
    fn set_read_timeout(&mut self, duration: Option<Duration>) -> io::Result<()> {
        self.inner.set_read_timeout(duration)
    }

    /// Set Write Timeout
    ///
    /// Set the write timeout for this `Tcp` by the specified duration. If None
    /// is passed as the duration the Transport will block indefinitely.
    ///
    /// # Errors
    /// Errors if a zero `Duration` is passed to this operation.
    ///
    /// # Examples
    ///```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn write_timeout_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     transport.set_write_timeout(Some(Duration::new(10, 0)))?;
    ///     Ok(())
    /// }
    /// ```
    fn set_write_timeout(&mut self, duration: Option<Duration>) -> io::Result<()> {
        self.inner.set_write_timeout(duration)
    }

    /// Peer Addr
    ///
    /// Obtain the peer address this `Tcp` is connected to.
    ///
    /// # Errors
    /// Will error if there is no peer connected to this object.
    ///
    /// # Examples
    ///```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn peer_addr_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     let peer = transport.peer_addr()?;
    ///     Ok(())
    /// }
    /// ```
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.inner.peer_addr()
    }

    /// Shutdown
    ///
    /// Shut down this `Tcp` as specified.
    ///
    /// # Errors
    /// This method may error if called on an invalid, already closed or
    /// disconnected connection.
    ///
    /// # Examples
    ///```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn shutdown_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     transport.shutdown(Shutdown::Both)?;
    ///     Ok(())
    /// }
    /// ```
    fn shutdown(&mut self, how: Shutdown) -> io::Result<()> {
        self.inner.shutdown(how)
    }
}

impl Write for Tcp {
    /// Write
    ///
    /// Write the buffer of provided bytes to the inner stream. This will
    /// return the number of bytes successfully written.
    ///
    /// # Errors
    /// This method will error if the underlying `TcpStream` raises an error.
    /// If this method errors, no data will be written. Note that it is not
    /// considered an error if the entire buffer could not be written.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn write_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     let mut buffer = "Hello World".as_bytes();
    ///     let count = transport.write(&mut buffer)?;
    ///     Ok(())
    /// }
    /// ```
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    /// Flush
    ///
    /// Flush the output stream, ensuring any buffered but not yet sent content
    /// reaches the destination.
    ///
    /// # Errors
    /// It is considered an error if not all bytes could be written due to I/O
    /// errors, or EOF being reached.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::transport::*;
    /// use std::net::TcpStream;
    ///
    /// pub fn flush_tcp_transport() -> std::io::Result<()> {
    ///     let mut transport = Tcp::new(TcpStream::connect("127.0.0.1")?);
    ///     let mut buffer = "Hello World".as_bytes();
    ///     transport.write(&mut buffer)?;
    ///     transport.flush();
    ///     Ok(())
    /// }
    /// ```
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

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
    /// Shut down this `Transport` as specified.
    ///
    /// # Errors
    /// Will return an `std::io::Error` if the `Transport` cannot be shutdown
    /// as requested.
    fn shutdown(&mut self, how: Shutdown) -> io::Result<()>;
}
