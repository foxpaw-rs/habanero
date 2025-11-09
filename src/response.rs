//! HTTP responses.
//! Todo(Paul): Module documentation.

pub use crate::http::Version;

/// HTTP Response Builder.
///
/// Utilizes the builder pattern to fluently construct a `Response`. Each
/// method call invalidates the previous `Builder`, and it is intended to be
/// chained from initial construction all the way to the finalize, `create`
/// method to create the `Response`. If multiple `Responses` are required based
/// off the same set of information, the `Builder` should be cloned.
///
/// # Examples
/// Todo(Paul): Examples once feature complete.
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    code: Code,
    version: Version,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Response::build` method to invoke the
    /// builder pattern and build up a `Response`.
    fn new(version: Version, code: Code) -> Self {
        Self { code, version }
    }

    /// Create the built `Response`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Response`.
    ///
    /// # Examples
    /// Todo(Paul): Examples once feature complete.
    #[must_use]
    pub fn create(self) -> Response {
        Response::new(self.version, self.code)
    }
}

/// A HTTP Response.
///
/// Stores information about the HTTP response, either received from a
/// connection (or `Client`), or built to be sent via a socket (or `Server`).
/// `Responses` are constructed using a builder pattern due to the nature of
///  the different information required to be contained within each `Response`.
///
/// # Examples
/// Todo(Paul): Examples once feature complete.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    code: Code,
    version: Version,
}

impl Response {
    /// Create a new `Response`.
    ///
    /// Creates a new response, invoked via the `Builder::create` method to
    /// finalise the construction of the `Response`.
    fn new(version: Version, code: Code) -> Self {
        Self { code, version }
    }

    /// Build a new `Response`.
    ///
    /// Creates a `Builder`, used to construct the `Response`. `Responses` are
    /// created using the builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //      Response,
    /// //      response::{
    /// //          Builder, Code, Version
    /// //      }
    /// //  };
    ///
    /// let builder = Response::build(Version::Http1_1, Code::Ok);
    /// ```
    #[must_use]
    pub fn build(version: Version, code: Code) -> Builder {
        Builder::new(version, code)
    }
}

/// The HTTP response codes.
///
/// Representation of the supported HTTP response codes used to specify the
/// type of response.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Code {
    // 1XX Informational Responses
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    // 2XX Successful Responses
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    // 3XX Redirection Messages
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4XX Client Error Responses
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
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    // 5XX Server Error Responses
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

#[cfg(test)]
mod tests {

    use super::*;

    // impl Builder

    #[test]
    // Todo(Paul): Add optional fields once implemented.
    fn builder_new_success() {
        let expected = Builder {
            code: Code::Ok,
            version: Version::Http1_1,
        };
        let actual = Builder::new(Version::Http1_1, Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    // Todo(Paul): Add optional fields once implemented.
    fn builder_create_success() {
        let expected = Response {
            code: Code::Ok,
            version: Version::Http1_1,
        };

        let actual = Builder::new(Version::Http1_1, Code::Ok).create();
        assert_eq!(expected, actual);
    }

    // impl Response

    #[test]
    // Todo(Paul): Add optional fields once implemented.
    fn response_new_success() {
        let expected = Response {
            code: Code::Ok,
            version: Version::Http1_1,
        };
        let actual = Response::new(Version::Http1_1, Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    // Todo(Paul): Add optional fields once implemented.
    fn response_build_success() {
        let expected = Builder {
            code: Code::Ok,
            version: Version::Http1_1,
        };
        let actual = Response::build(Version::Http1_1, Code::Ok);
        assert_eq!(expected, actual);
    }
}
