//! Connection
//!
//! Todo: Module documentation

use crate::common::{Request, Response};
use crate::err::ConnectionError;

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
}
