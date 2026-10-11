//! Connection
//!
//! Todo: Module documentation

use crate::codec;
use crate::common::{Request, Response};
use crate::err::ConnectionError;
use crate::transport::Transport;
use std::io::BufReader;

/// `Client`
///
/// The `Client` trait is responsible for defining the methods used by the
/// connection tied to a [`Client`].
pub trait Client {
    /// Send Request
    ///
    /// Send a [`Request`] to a remote host, receiving a `Response`.
    ///
    /// # Errors
    /// This method will return a `ConnectionError` with a variant based on the
    /// manner in which the error occurred:
    /// * `ConnectionError::Parse`: If there is an error with the parsing; or
    /// * `ConnectionError::Serialize`: If there is an error with the serialization
    ///   process.
    fn send_request(&mut self, request: Request) -> Result<Response, ConnectionError>;
}

/// `Http1Client`
///
/// The `Http1Client` is a connection responsible for handling client
/// connections to a remote host over a generic [`Transport`] type `T`.
///
/// This would generally be used by the [`Client`] and only utilised in 
/// isolation in rare cases.
///
/// # Examples
/// ```rust, no_run
/// use foxpaw_habanero::*;
/// use foxpaw_habanero::connection::{Client, Http1Client};
/// use foxpaw_habanero::transport::Tcp;
/// use std::net::TcpStream;
///
/// fn send() -> Result<Response, String> {
///     let stream = TcpStream::connect("127.0.0.1")
///         .map_err(|e| e.to_string())?;
///     let mut transport = Tcp::new(stream);
///     let request = Request::new(Method::Get, "/", Version::Http11);
///     let mut client_connection = Http1Client::new(transport);
///     let response = client_connection.send_request(request).map_err(|e| e.to_string())?;
///     Ok(response)
/// }
/// ```
#[derive(Debug)]
pub struct Http1Client<T: Transport> {
    transport: T,
}

impl<T> Http1Client<T>
where
    T: Transport,
{
    /// New
    ///
    /// Create a new `Http1Client` connection over the provided [`Transport`].
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::connection::Http1Client;
    /// use foxpaw_habanero::transport::Tcp;
    /// use std::net::TcpStream;
    ///
    /// pub fn new_http1_client_connection() -> Result<(), String> {
    ///     let stream = TcpStream::connect("127.0.0.1")
    ///             .map_err(|e| e.to_string())?;
    ///     let transport = Tcp::new(stream);
    ///     let connection = Http1Client::new(transport);
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> Client for Http1Client<T>
where
    T: Transport + std::fmt::Debug,
{
    /// Send Request
    ///
    /// Send a [`Request`] to a remote host, receiving a `Response`.
    ///
    /// # Errors
    /// This method will return a `ConnectionError` with a variant based on the
    /// manner in which the error occurred:
    /// * `ConnectionError::Parse`: If there is an error with the parsing; or
    /// * `ConnectionError::Serialize`: If there is an error with the serialization
    ///   process.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::connection::{Client, Http1Client};
    /// use foxpaw_habanero::transport::Tcp;
    /// use std::net::TcpStream;
    ///
    /// fn send() -> Result<Response, String> {
    ///     let stream = TcpStream::connect("127.0.0.1")
    ///         .map_err(|e| e.to_string())?;
    ///     let mut transport = Tcp::new(stream);
    ///     let request = Request::new(Method::Get, "/", Version::Http11);
    ///     let mut client_connection = Http1Client::new(transport);
    ///     let response = client_connection.send_request(request).map_err(|e| e.to_string())?;
    ///     Ok(response)
    /// }
    /// ```
    fn send_request(&mut self, request: Request) -> Result<Response, ConnectionError> {
        codec::Http1::serialize_request(&mut self.transport, &request)?;

        let mut reader = BufReader::new(&mut self.transport);
        Ok(codec::Http1::parse_response(&mut reader)?)
    }
}

/// `Server`
///
/// The `Server` trait is responsible for defining the methods used by the
/// connection tied to a [`Server`].
pub trait Server {
    /// Next Request
    ///
    /// Obtain the next [`Request`] waiting on this `ServerConnection`. Will
    /// return either Ok(Some(Request)) if a request was obtained or Ok(None)
    /// if the operation was successful but no request was waiting.
    ///
    /// # Errors
    /// This method will return a `ConnectionError::Parse` if there is an error
    /// with the parsing of the incoming HTTP request.
    fn next_request(&mut self) -> Result<Option<Request>, ConnectionError>;

    /// Send Response
    ///
    /// Serialize and return the provided [`Response`] back over the
    /// connection.
    ///
    /// # Errors
    /// This method will return a `ConnectionError::Serialize` if there is an
    /// error with the serialization of the provided [`Response`];
    fn send_response(&mut self, response: Response) -> Result<(), ConnectionError>;

    /// Is Persistent
    ///
    /// Determine if this connection is kept alive and handles multiple
    /// transmissions.
    fn is_persistent(&self) -> bool;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::common::{Method, Version};
    use crate::err::{ParseError, SerializeError};
    use crate::mock::MockTransport;

    #[test]
    fn http1_client_new_correct() {
        let actual = Http1Client::new(MockTransport::new(b""));
        let expected = Http1Client {
            transport: MockTransport::new(b"")
        };
        assert_eq!(actual.transport, expected.transport);
    }

    #[test]
    fn http1_client_send_request_correct() {
        let mut connection = Http1Client::new(MockTransport::new(b"HTTP/1.1 200 OK\r\n\r\n"));
        let request = Request::new(Method::Get, "/", Version::Http11);
        
        let actual = connection.send_request(request);
        let expected = Response::new(Version::Http11, 200);

        assert_eq!(actual, Ok(expected));
        assert_eq!(connection.transport.write_buffer(), b"GET / HTTP/1.1\r\n\r\n");
    }

    #[test]
    fn http1_client_send_request_serialise_error() {
        let mut connection = Http1Client::new(MockTransport::new(b"").close_connection());
        let request = Request::new(Method::Get, "/", Version::Http11);

        let actual = connection.send_request(request);
        let expected = ConnectionError::Serialize(SerializeError::Io("failed to write whole buffer".to_string()));

        assert_eq!(actual, Err(expected));
    }

    #[test]
    fn http1_client_send_request_parse_error() {
        let mut connection = Http1Client::new(MockTransport::new(b""));
        let request = Request::new(Method::Get, "/", Version::Http11);

        let actual = connection.send_request(request);
        let expected = ConnectionError::Parse(ParseError::Empty(
            "Connection closed, unable to read data".to_string(),
        ));

        assert_eq!(actual, Err(expected));
    }
}
