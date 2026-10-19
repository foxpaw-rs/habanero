//! Error
//!
//! This module probides the error types utilised by the crate, along with
//! assistance implementations to convert between error types when relevant.
//!
//! # Architecture
//! The error types defined for this crate are:
//! * [`ParseError`]; and
//! * [`SerializeError`].
//!
//! # Examples
//! ```rust
//! use foxpaw_habanero::err::ParseError;
//!
//! fn my_parsing_method(input: &str) -> Result<(), ParseError> {
//!     // Doing some parsing
//!     if input.is_empty() {
//!         return Err(ParseError::Empty("The input cannot be empty".to_string()));
//!     }
//!     Ok(())
//!}
//! ```

use std::error::Error;
use std::fmt;
use std::io::{self, ErrorKind};

/// `ClientError`
///
/// The enumeration of available client error types. Each variant houses an
/// inner context to be provided by the user of the error.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::err::*;
///
/// let error = ClientError::Aborted("Something went wrong".to_string());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ClientError {
    Connection(ConnectionError),
    Refused(String),
    TimedOut(String),
    Aborted(String),
    Reset(String),
    Unreachable(String),
    AddrInUse(String),
    AddrNotAvailable(String),
    PermissionDenied(String),
    Other(String),
}

impl fmt::Display for ClientError {
    /// Fmt
    ///
    /// Format the `ClientError` for Display.
    ///
    /// # Error
    /// This method will error if the underlying call to write errors.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::err::*;
    ///
    /// let error = ClientError::Reset(String::new());
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Connection(inner) => write!(f, "ClientError::Connection => {inner}"),
            Self::Refused(inner) if !inner.is_empty() => {
                write!(f, "ClientError::Refused => {inner}")
            }
            Self::Refused(_) => write!(f, "ClientError::Refused"),
            Self::TimedOut(inner) if !inner.is_empty() => {
                write!(f, "ClientError::TimedOut => {inner}")
            }
            Self::TimedOut(_) => write!(f, "ClientError::TimedOut"),
            Self::Aborted(inner) if !inner.is_empty() => {
                write!(f, "ClientError::Aborted => {inner}")
            }
            Self::Aborted(_) => write!(f, "ClientError::Aborted"),
            Self::Reset(inner) if !inner.is_empty() => write!(f, "ClientError::Reset => {inner}"),
            Self::Reset(_) => write!(f, "ClientError::Reset"),
            Self::Unreachable(inner) if !inner.is_empty() => {
                write!(f, "ClientError::Unreachable => {inner}")
            }
            Self::Unreachable(_) => write!(f, "ClientError::Unreachable"),
            Self::AddrInUse(inner) if !inner.is_empty() => {
                write!(f, "ClientError::AddrInUse => {inner}")
            }
            Self::AddrInUse(_) => write!(f, "ClientError::AddrInUse"),
            Self::AddrNotAvailable(inner) if !inner.is_empty() => {
                write!(f, "ClientError::AddrNotAvailable => {inner}")
            }
            Self::AddrNotAvailable(_) => write!(f, "ClientError::AddrNotAvailable"),
            Self::PermissionDenied(inner) if !inner.is_empty() => {
                write!(f, "ClientError::PermissionDenied => {inner}")
            }
            Self::PermissionDenied(_) => write!(f, "ClientError::PermissionDenied"),
            Self::Other(inner) if !inner.is_empty() => {
                write!(f, "ClientError::Other => {inner}")
            }
            Self::Other(_) => write!(f, "ClientError::Other"),
        }
    }
}

impl Error for ClientError {}

impl From<ConnectionError> for ClientError {
    /// From
    ///
    /// Convert from a [`ConnectionError`] into a `ClientError` by wrapping
    /// it in the `ClientError::Connection` variant.
    fn from(e: ConnectionError) -> Self {
        Self::Connection(e)
    }
}

impl From<io::Error> for ClientError {
    /// From
    ///
    /// Convert from an `io::Error` into a `ClientError` by wrapping
    /// it in the relevant variant.
    fn from(e: io::Error) -> Self {
        match e.kind() {
            ErrorKind::ConnectionRefused => Self::Refused(e.to_string()),
            ErrorKind::TimedOut => Self::TimedOut(e.to_string()),
            ErrorKind::ConnectionAborted => Self::Aborted(e.to_string()),
            ErrorKind::ConnectionReset => Self::Reset(e.to_string()),
            ErrorKind::HostUnreachable | ErrorKind::NetworkUnreachable => {
                Self::Unreachable(e.to_string())
            }
            ErrorKind::AddrInUse => Self::AddrInUse(e.to_string()),
            ErrorKind::AddrNotAvailable => Self::AddrNotAvailable(e.to_string()),
            ErrorKind::PermissionDenied => Self::PermissionDenied(e.to_string()),
            _ => Self::Other(e.to_string()),
        }
    }
}

/// `ConnectionError`
///
/// The enumeration of available connection error types. Each variant houses an
/// inner `String` context to be provided by the user of the error.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::err::*;
///
/// fn my_connect_method(input: &str) -> Result<(), ConnectionError> {
///     if input.is_empty() {
///         return Err(ConnectionError::Parse(
///             ParseError::Empty("More context".to_string())
///         ));
///     }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConnectionError {
    Parse(ParseError),
    Serialize(SerializeError),
}

impl fmt::Display for ConnectionError {
    /// Fmt
    ///
    /// Format the `ConnectionError` for Display.
    ///
    /// # Error
    /// This method will error if the underlying call to write errors.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::err::*;
    ///
    /// let error = ConnectionError::Parse(
    ///     ParseError::Invalid("Invalid value found when parsing".to_string())
    /// );
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(inner) => write!(f, "ConnectionError::Parse => {inner}"),
            Self::Serialize(inner) => write!(f, "ConnectionError::Serialize => {inner}"),
        }
    }
}

impl Error for ConnectionError {}

impl From<ParseError> for ConnectionError {
    /// From
    ///
    /// Convert from a [`ParseError`] into a `ConnectionError` by wrapping it
    /// in the `ConnectionError::Parse` variant.
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<SerializeError> for ConnectionError {
    /// From
    ///
    /// Convert from a [`SerializeError`] into a `ConnectionError` by wrapping
    /// it in the `ConnectionError::Serialize` variant.
    fn from(e: SerializeError) -> Self {
        Self::Serialize(e)
    }
}

/// `HttpError`
///
/// The enumeration of available server error types. Each variant represents
/// a 4xx or 5xx HTTP response.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::*;
/// use foxpaw_habanero::err::HttpError;
///
/// fn check_version(req: Request) -> Result<Request, HttpError> {
///     match req.version {
///         Version::Http11 => Ok(req),
///         _ => Err(HttpError::HttpVersionNotSupported),
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HttpError {
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalHttpError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl fmt::Display for HttpError {
    /// Fmt
    ///
    /// Format the `HttpError` for Display.
    ///
    /// # Error
    /// This method will error if the underlying call to write errors.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::err::HttpError;
    ///
    /// let error = HttpError::BadRequest;
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadRequest => write!(f, "HttpError::BadRequest => {}", *self as u16),
            Self::Unauthorized => write!(f, "HttpError::Unauthorized => {}", *self as u16),
            Self::PaymentRequired => write!(f, "HttpError::PaymentRequired => {}", *self as u16),
            Self::Forbidden => write!(f, "HttpError::Forbidden => {}", *self as u16),
            Self::NotFound => write!(f, "HttpError::NotFound => {}", *self as u16),
            Self::MethodNotAllowed => {
                write!(f, "HttpError::MethodNotAllowed => {}", *self as u16)
            }
            Self::NotAcceptable => write!(f, "HttpError::NotAcceptable => {}", *self as u16),
            Self::ProxyAuthenticationRequired => write!(
                f,
                "HttpError::ProxyAuthenticationRequired => {}",
                *self as u16
            ),
            Self::RequestTimeout => write!(f, "HttpError::RequestTimeout => {}", *self as u16),
            Self::Conflict => write!(f, "HttpError::Conflict => {}", *self as u16),
            Self::Gone => write!(f, "HttpError::Gone => {}", *self as u16),
            Self::LengthRequired => write!(f, "HttpError::LengthRequired => {}", *self as u16),
            Self::PreconditionFailed => {
                write!(f, "HttpError::PreconditionFailed => {}", *self as u16)
            }
            Self::ContentTooLarge => write!(f, "HttpError::ContentTooLarge => {}", *self as u16),
            Self::UriTooLong => write!(f, "HttpError::UriTooLong => {}", *self as u16),
            Self::UnsupportedMediaType => {
                write!(f, "HttpError::UnsupportedMediaType => {}", *self as u16)
            }
            Self::RangeNotSatisfiable => {
                write!(f, "HttpError::RangeNotSatisfiable => {}", *self as u16)
            }
            Self::ExpectationFailed => {
                write!(f, "HttpError::ExpectationFailed => {}", *self as u16)
            }
            Self::ImATeapot => write!(f, "HttpError::ImATeapot => {}", *self as u16),
            Self::MisdirectedRequest => {
                write!(f, "HttpError::MisdirectedRequest => {}", *self as u16)
            }
            Self::TooEarly => write!(f, "HttpError::TooEarly => {}", *self as u16),
            Self::UpgradeRequired => write!(f, "HttpError::UpgradeRequired => {}", *self as u16),
            Self::PreconditionRequired => {
                write!(f, "HttpError::PreconditionRequired => {}", *self as u16)
            }
            Self::TooManyRequests => write!(f, "HttpError::TooManyRequests => {}", *self as u16),
            Self::RequestHeaderFieldsTooLarge => write!(
                f,
                "HttpError::RequestHeaderFieldsTooLarge => {}",
                *self as u16
            ),
            Self::UnavailableForLegalReasons => write!(
                f,
                "HttpError::UnavailableForLegalReasons => {}",
                *self as u16
            ),
            Self::InternalHttpError => {
                write!(f, "HttpError::InternalHttpError => {}", *self as u16)
            }
            Self::NotImplemented => write!(f, "HttpError::NotImplemented => {}", *self as u16),
            Self::BadGateway => write!(f, "HttpError::BadGateway => {}", *self as u16),
            Self::ServiceUnavailable => {
                write!(f, "HttpError::ServiceUnavailable => {}", *self as u16)
            }
            Self::GatewayTimeout => write!(f, "HttpError::GatewayTimeout => {}", *self as u16),
            Self::HttpVersionNotSupported => {
                write!(f, "HttpError::HttpVersionNotSupported => {}", *self as u16)
            }
            Self::VariantAlsoNegotiates => {
                write!(f, "HttpError::VariantAlsoNegotiates => {}", *self as u16)
            }
            Self::NotExtended => write!(f, "HttpError::NotExtended => {}", *self as u16),
            Self::NetworkAuthenticationRequired => write!(
                f,
                "HttpError::NetworkAuthenticationRequired => {}",
                *self as u16
            ),
        }
    }
}

impl Error for HttpError {}

/// `ParseError`
///
/// The enumeration of available parsing error types. Each variant houses an
/// inner `String` context to be provided by the user of the error.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::err::ParseError;
///
/// fn my_parsing_method(input: &str) -> Result<(), ParseError> {
///     if input.is_empty() {
///         return Err(ParseError::Empty("The input cannot be empty".to_string()));
///     }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    Empty(String),
    Invalid(String),
    Malformed(String),
}

impl fmt::Display for ParseError {
    /// Fmt
    ///
    /// Format the `ParseError` for Display.
    ///
    /// # Error
    /// This method will error if the underlying call to write errors.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::err::ParseError;
    ///
    /// let error = ParseError::Invalid("Invalid value found when parsing".to_string());
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty(inner) if !inner.is_empty() => write!(f, "ParseError::Empty => {inner}"),
            Self::Empty(_) => write!(f, "ParseError::Empty"),
            Self::Invalid(inner) if !inner.is_empty() => {
                write!(f, "ParseError::Invalid => {inner}")
            }
            Self::Invalid(_) => write!(f, "ParseError::Invalid"),
            Self::Malformed(inner) if !inner.is_empty() => {
                write!(f, "ParseError::Malformed => {inner}")
            }
            Self::Malformed(_) => write!(f, "ParseError::Malformed"),
        }
    }
}

impl Error for ParseError {}

/// `SerializeError`
///
/// The enumeration of available serialization error types. Each variant houses
/// an inner `String` context to be provided by the user of the error.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::err::SerializeError;
///
/// fn my_serialize_method(input: &str) -> Result<(), SerializeError> {
///     if input.is_empty() {
///         return Err(SerializeError::Io("Writing Error.".to_string()));
///     }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SerializeError {
    Io(String),
}

impl fmt::Display for SerializeError {
    /// Fmt
    ///
    /// Format the `SerializeError` for Display.
    ///
    /// # Error
    /// This method will error if the underlying call to write errors.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::err::SerializeError;
    ///
    /// let error = SerializeError::Io("Unable to write".to_string());
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(inner) if !inner.is_empty() => write!(f, "SerializeError::Io => {inner}"),
            Self::Io(_) => write!(f, "SerializeError::Io"),
        }
    }
}

impl Error for SerializeError {}

/// `ServerError`
///
/// The enumeration of available server error types. Each variant houses an
/// inner `String` context to be provided by the user of the error.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::err::ServerError;
///
/// fn try_listen(input: &str) -> Result<(), ServerError> {
///     if input != "127.0.0.1:8080" {
///         return Err(ServerError::Bind("Unable to bind to that port".to_string()));
///     }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ServerError {
    AddrInUse(String),
    Bind(String),
    Invalid(String),
    Other(String),
}

impl fmt::Display for ServerError {
    /// Fmt
    ///
    /// Format the `ServerError` for Display.
    ///
    /// # Error
    /// This method will error if the underlying call to write errors.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::err::ServerError;
    ///
    /// let error = ServerError::Bind("Unable to bind to that port".to_string());
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AddrInUse(inner) if !inner.is_empty() => {
                write!(f, "ServerError::AddrInUse => {inner}")
            }
            Self::AddrInUse(_) => write!(f, "ServerError::AddrInUse"),
            Self::Bind(inner) if !inner.is_empty() => write!(f, "ServerError::Bind => {inner}"),
            Self::Bind(_) => write!(f, "ServerError::Bind"),
            Self::Invalid(inner) if !inner.is_empty() => {
                write!(f, "ServerError::Invalid => {inner}")
            }
            Self::Invalid(_) => write!(f, "ServerError::Invalid"),
            Self::Other(inner) if !inner.is_empty() => write!(f, "ServerError::Other => {inner}"),
            Self::Other(_) => write!(f, "ServerError::Other"),
        }
    }
}

impl Error for ServerError {}

impl From<io::Error> for ServerError {
    /// From
    ///
    /// Convert from an `io::Error` into a `ServerError` by wrapping
    /// it in the relevant variant.
    fn from(e: io::Error) -> Self {
        match e.kind() {
            ErrorKind::InvalidInput => ServerError::Invalid(e.to_string()),
            ErrorKind::PermissionDenied => ServerError::Bind(e.to_string()),
            ErrorKind::AddrInUse => ServerError::AddrInUse(e.to_string()),
            _ => ServerError::Other(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // impl Display for ClientError

    #[test]
    fn client_error_fmt_correct() {
        let expected = "ClientError::AddrInUse => Custom context".to_string();
        let actual = ClientError::AddrInUse("Custom context".to_string()).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn client_error_fmt_empty() {
        let expected = "ClientError::AddrInUse".to_string();
        let actual = ClientError::AddrInUse(String::new()).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn client_error_fmt_nested() {
        let expected = "ClientError::Connection => \
            ConnectionError::Parse => \
            ParseError::Empty => \
            Custom context"
            .to_string();
        let actual = ClientError::Connection(ConnectionError::Parse(ParseError::Empty(
            "Custom context".to_string(),
        )))
        .to_string();
        assert_eq!(actual, expected);
    }

    // impl From<ConnectionError> for ClientError

    #[test]
    fn client_error_from_connection_error() {
        let actual = ConnectionError::Parse(ParseError::Empty(String::new()));
        let expected =
            ClientError::Connection(ConnectionError::Parse(ParseError::Empty(String::new())));
        assert_eq!(ClientError::from(actual), expected)
    }

    // impl From<io::Error> for ClientError

    #[test]
    fn client_error_from_io_error() {
        let actual = Into::<io::Error>::into(ErrorKind::AddrInUse);
        let expected = ClientError::AddrInUse("address in use".to_string());
        assert_eq!(ClientError::from(actual), expected)
    }

    // impl Display for ConnectionError

    #[test]
    fn connection_error_fmt_correct() {
        let expected = "ConnectionError::Parse => \
            ParseError::Invalid => \
            Custom context"
            .to_string();
        let actual =
            ConnectionError::Parse(ParseError::Invalid("Custom context".to_string())).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn connection_error_fmt_empty() {
        let expected = "ConnectionError::Parse => \
            ParseError::Invalid"
            .to_string();
        let actual = ConnectionError::Parse(ParseError::Invalid(String::new())).to_string();
        assert_eq!(actual, expected);
    }

    // impl From<ParseError> for ConnectionError

    #[test]
    fn connection_error_from_parse_error() {
        let actual = ParseError::Empty("".to_string());
        let expected = ConnectionError::Parse(ParseError::Empty("".to_string()));
        assert_eq!(ConnectionError::from(actual), expected)
    }

    // impl From<SerializeError> for ConnectionError

    #[test]
    fn connection_error_from_serialize_error() {
        let actual = SerializeError::Io("".to_string());
        let expected = ConnectionError::Serialize(SerializeError::Io("".to_string()));
        assert_eq!(ConnectionError::from(actual), expected)
    }

    // impl Display for HttpError

    #[test]
    fn http_error_fmt_correct() {
        let expected = "HttpError::NotFound => 404".to_string();
        let actual = HttpError::NotFound.to_string();
        assert_eq!(actual, expected);
    }

    // impl Display for ParseError

    #[test]
    fn parse_error_fmt_correct() {
        let expected = "ParseError::Empty => Custom context".to_string();
        let actual = ParseError::Empty("Custom context".to_string()).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_error_fmt_empty() {
        let expected = "ParseError::Invalid".to_string();
        let actual = ParseError::Invalid(String::new()).to_string();
        assert_eq!(actual, expected);
    }

    // impl Display for SerializeError

    #[test]
    fn serialize_error_fmt_correct() {
        let expected = "SerializeError::Io => Custom context".to_string();
        let actual = SerializeError::Io("Custom context".to_string()).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialize_error_fmt_empty() {
        let expected = "SerializeError::Io".to_string();
        let actual = SerializeError::Io(String::new()).to_string();
        assert_eq!(actual, expected);
    }

    // impl Display for ServerError

    #[test]
    fn server_error_fmt_correct() {
        let expected = "ServerError::Bind => Unable to connect to port".to_string();
        let actual = ServerError::Bind("Unable to connect to port".to_string()).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn server_error_fmt_empty() {
        let expected = "ServerError::Bind".to_string();
        let actual = ServerError::Bind(String::new()).to_string();
        assert_eq!(actual, expected);
    }

    // impl From<io::Error> for ServerError

    #[test]
    fn server_error_from_io_error() {
        let actual = Into::<io::Error>::into(ErrorKind::AddrInUse);
        let expected = ServerError::AddrInUse("address in use".to_string());
        assert_eq!(ServerError::from(actual), expected)
    }
}
