//! HTTP/1.1 responses.
//!
//! Todo(Paul): Module documentation

/// HTTP Response Builder.
///
/// Utilizes the builder pattern to fluently construct a `Response`. Each
/// method call invalidates the previous `Builder`, and it intended to be
/// chained from initial construction all the way to finalization. If multiple
/// `Responses` are required based on the same set of information, the
/// `Builder` should be cloned.
///
/// # Examples
/// ```rust
/// use habanero::http1::*;
///
/// // Todo(Paul): Add more once body and headers implemented.
/// let response = Response::build(Code::Ok)
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    code: Code,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Response::build` method to invoke the
    /// builder pattern and build up a `Response`.
    fn new(code: Code) -> Self {
        Self { code }
    }

    /// Create the built `Response`.
    ///
    /// Finalizes the `Builder`, invalidating the current reference and
    /// creating the built `Response`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// // Todo(Paul): Add more once body and headers implemented.
    /// let response = Response::build(Code::Ok)
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Response {
        Response::new(self.code)
    }
}

/// An HTTP Response.
///
/// Stores information about the HTTP response, either received from a
/// connection (or `Client`), or built to be sent via a socket (or `Server`).
/// `Responses` are constructed using a builder pattern due to the nature of
/// the optional information required to be contained within each `Response`.
///
/// # Examples
/// ```rust
/// use habanero::http1::*;
///
/// // Todo(Paul): Add more once body and headers implemented.
/// let response = Response::build(Code::Ok)
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    code: Code,
}

impl Response {
    /// Create a new `Response`.
    ///
    /// Creates a new response, invoked via the `Builder::create` method to
    /// finalize the construction of the `Response`.
    fn new(code: Code) -> Self {
        Self { code }
    }

    /// Build a new `Response`.
    ///
    /// Creates a `Builder`, used to construct the `Response`. `Responses` are
    /// created using the builder pattern.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let builder = Response::build(Code::Ok);
    /// ```
    #[must_use]
    pub fn build(code: Code) -> Builder {
        Builder::new(code)
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
    fn builder_new_success() {
        let expected = Builder { code: Code::Ok };
        let actual = Builder::new(Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Response { code: Code::Ok };
        let actual = Builder::new(Code::Ok).create();
        assert_eq!(expected, actual);
    }

    // impl Response

    #[test]
    fn response_new_success() {
        let expected = Response { code: Code::Ok };
        let actual = Response::new(Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_build_success() {
        let expected = Builder { code: Code::Ok };
        let actual = Response::build(Code::Ok);
        assert_eq!(expected, actual);
    }
}
