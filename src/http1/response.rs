//! HTTP/1.1 Responses.
//!
//! # Response format.
//! As an HTTP response format has a number of optional fields, a `Response` is
//! initially built via a `Builder`. This allows for the addition of optional
//! fields without requiring the `Response` to be mutable at any point.
//!
//! The HTTP response format requires a version and status code. Headers and the
//! response body are optional. For example both the following are valid HTTP
//! responses.
//!
//! ```text
//! // Without headers and a response body.
//! HTTP/1.1 200 OK
//!
//! // Providing headers and a response body.
//! HTTP/1.1 200 OK
//! Content-Type: text/plain
//! Content-Length: 11
//!
//! Hello World
//! ```
//!
//! As a status code is required, it  must be initially passed to the build
//! method on `Response`. Headers and a body can then be added by calling the
//! relevant methods on the `Builder`. The same responses above would be
//! constructed as so.
//!
//! ```rust
//! use habanero::http1::*;
//!
//! // Without headers and a response body.
//! Response::build(Code::Ok).create();
//!
//! // Providing headers and a response body.
//! Response::build(Code::Ok)
//!     .header("Content-Type", "text/plain")
//!     .header("Content-Length", "11")
//!     .body("Hello World")
//!     .create();
//! ```
//!
//! The building process also provides shortcut methods for setting the response
//! to contain json, html or form url-encoded data, by also setting the
//! `Content-Type` and `Content-Length` header fields.
//!
//! ```rust
//! use habanero::http1::*;
//!
//! let json = Response::build(Code::Ok)
//!     .json("{ }")
//!     .create();
//!
//! let html = Response::build(Code::Ok)
//!     .html("<html></html>")
//!     .create();
//!
//! let url = Response::build(Code::Ok)
//!     .url_encoded("key=value")
//!     .create();
//! ```
//!
//! # Accessing Response data
//! To access the internal Response data once constructed, access methods are
//! provided on the Response type itself.
//!
//! ```rust
//! use habanero::http1::*;
//!
//! let response = Response::build(Code::Ok)
//!     .create();
//! let code = response.code();
//! ```

use core::fmt::{self, Debug, Display, Formatter};
use std::collections::BTreeMap;

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
/// let response = Response::build(Code::Ok)
///     .header("Content-Type", "text/plain")
///     .body("Hello World")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    body: String,
    code: Code,
    headers: BTreeMap<String, String>,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Response::build` method to invoke the
    /// builder pattern and build up a `Response`.
    fn new(code: Code) -> Self {
        Self {
            body: String::new(),
            code,
            headers: BTreeMap::new(),
        }
    }

    /// Set a `Response` body.
    ///
    /// Set a body on the `Response`. This will overwrite any previously set
    /// value.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .body("Hello World")
    ///     .create();
    /// ```
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
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
    /// let response = Response::build(Code::Ok)
    ///     .header("Content-Type", "text/plain")
    ///     .body("Hello World")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Response {
        Response::new(self.code, self.headers, self.body)
    }

    /// Set a `Response` header.
    ///
    /// Set a header on the `Response`. This will overwrite any previously set
    /// value for that header key.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .header("Content-Type", "text/plain")
    ///     .create();
    /// ```
    #[must_use]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set a `Response` HTML body.
    ///
    /// Set an HTML body on the `Response`. This will overwrite any previously
    /// set value for the response body, Content-Type header and Content-Length
    /// header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .html("<html></html>")
    ///     .create();
    /// ```
    #[must_use]
    pub fn html(self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();
        self.body(body)
            .header("Content-Type", "text/html")
            .header("Content-Length", len)
    }

    /// Set a `Response` JSON body.
    ///
    /// Set a JSON body on the `Response`. This will overwrite any previously
    /// set value for the response body, Content-Type header and Content-Length
    /// header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .json("{ }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn json(self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();
        self.body(body)
            .header("Content-Type", "application/json")
            .header("Content-Length", len)
    }

    /// Set a `Response` url encoded body.
    ///
    /// Set a url encoded body on the `Response`. This will overwrite any
    /// previously set value for the response body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .url_encoded("key=value")
    ///     .create();
    /// ```
    #[must_use]
    pub fn url_encoded(self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();
        self.body(body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", len)
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
/// let response = Response::build(Code::Ok)
///     .header("Content-Type", "text/plain")
///     .body("Hello World")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    body: String,
    code: Code,
    headers: BTreeMap<String, String>,
}

impl Response {
    /// Create a new `Response`.
    ///
    /// Creates a new response, invoked via the `Builder::create` method to
    /// finalize the construction of the `Response`.
    fn new(code: Code, headers: BTreeMap<String, String>, body: String) -> Self {
        Self {
            body,
            code,
            headers,
        }
    }

    /// Retrieve the `Response` body.
    ///
    /// Retrieve an immutable reference to the body stored in the `Response`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .body("Hello World")
    ///     .create();
    /// let body = response.body();
    /// ```
    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
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

    /// Retrieve the `Response` code.
    ///
    /// Retrieve an immutable reference to the code stored in the `Response`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .create();
    /// let code = response.code();
    /// ```
    #[must_use]
    pub fn code(&self) -> &Code {
        &self.code
    }

    /// Retrieve the requested `Response` header.
    ///
    /// Retrieve an immutable reference to the requested header stored in the
    /// `Response`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .header("Content-Type", "text/plain")
    ///     .create();
    /// let header = response.header("Content-Type");
    /// ```
    #[must_use]
    pub fn header(&self, key: impl Into<String>) -> Option<&str> {
        self.headers.get(&key.into()).map(String::as_str)
    }

    /// Retrieve the `Response` headers.
    ///
    /// Retrieve an immutable reference to the headers stored in the
    /// `Response`.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .header("Content-Type", "text/plain")
    ///     .create();
    /// let headers = response.headers();
    /// ```
    #[must_use]
    pub fn headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }
}

impl Display for Response {
    /// Format the `Response`.
    ///
    /// Formats the `Response` into an HTTP compatible response format, able to
    /// be sent to a server.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::*;
    ///
    /// let response = Response::build(Code::Ok)
    ///     .header("Content-Type", "text/plain")
    ///     .body("Hello World")
    ///     .create();
    /// let string = response.to_string();
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "HTTP/1.1 {}\n{}\n{}",
            self.code,
            self.headers.iter().fold(String::new(), |fold, pair| {
                format!("{fold}{}: {}\n", pair.0, pair.1)
            }),
            self.body
        )
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

impl Display for Code {
    /// Format the `Code`.
    ///
    /// Formats the `Code` into what would be expected for an HTTP response.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http1::Code;
    ///
    /// let code = Code::Ok;
    /// let string = code.to_string();
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let readable = match self {
            Code::Continue => "Continue",
            Code::SwitchingProtocols => "Switching Protocols",
            Code::Processing => "Processing",
            Code::EarlyHints => "Early Hints",

            // 2XX Successful Responses
            Code::Ok => "OK",
            Code::Created => "Created",
            Code::Accepted => "Accepted",
            Code::NonAuthoritativeInformation => "Non-Authoritative Information",
            Code::NoContent => "No Content",
            Code::ResetContent => "Reset Content",
            Code::PartialContent => "Partial Content",
            Code::MultiStatus => "Multi-Status",
            Code::AlreadyReported => "Already Reported",
            Code::IMUsed => "IM Used",

            // 3XX Redirection Messages
            Code::MultipleChoices => "Multiple Choices",
            Code::MovedPermanently => "Moved Permanently",
            Code::Found => "Found",
            Code::SeeOther => "See Other",
            Code::NotModified => "Not Modified",
            Code::TemporaryRedirect => "Temporary Redirect",
            Code::PermanentRedirect => "Permanent Redirect",

            // 4XX Client Error Responses
            Code::BadRequest => "Bad Request",
            Code::Unauthorized => "Unauthorized",
            Code::PaymentRequired => "Payment Required",
            Code::Forbidden => "Forbidden",
            Code::NotFound => "Not Found",
            Code::MethodNotAllowed => "Method Not Allowed",
            Code::NotAcceptable => "Not Acceptable",
            Code::ProxyAuthenticationRequired => "Proxy Authentication Required",
            Code::RequestTimeout => "Request Timeout",
            Code::Conflict => "Conflict",
            Code::Gone => "Gone",
            Code::LengthRequired => "Length Required",
            Code::PreconditionFailed => "Precondition Failed",
            Code::ContentTooLarge => "Content Too Large",
            Code::UriTooLong => "Uri Too Long",
            Code::UnsupportedMediaType => "Unsupported Media Type",
            Code::RangeNotSatisfiable => "Range Not Satisfiable",
            Code::ExpectationFailed => "Expectation Failed",
            Code::ImATeapot => "I'm a teapot",
            Code::MisdirectedRequest => "Misdirected Request",
            Code::UnprocessableContent => "Unprocessable Content",
            Code::Locked => "Locked",
            Code::FailedDependency => "Failed Dependency",
            Code::TooEarly => "Too Early",
            Code::UpgradeRequired => "Upgrade Required",
            Code::PreconditionRequired => "Precondition Required",
            Code::TooManyRequests => "Too Many Requests",
            Code::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Code::UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            // 5XX Server Error Responses
            Code::InternalServerError => "Internal Server Error",
            Code::NotImplemented => "Not Implemented",
            Code::BadGateway => "Bad Gateway",
            Code::ServiceUnavailable => "Service Unavailable",
            Code::GatewayTimeout => "Gateway Timeout",
            Code::HTTPVersionNotSupported => "Http Version Not Supported",
            Code::VariantAlsoNegotiates => "Variant Also Negotiates",
            Code::InsufficientStorage => "Insufficient Storage",
            Code::LoopDetected => "Loop Detected",
            Code::NotExtended => "Not Extended",
            Code::NetworkAuthenticationRequired => "Network Authentication Required",
        };
        let code = *self as u16;
        write!(f, "{code} {readable}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // impl Builder

    #[test]
    fn builder_new_success() {
        let expected = Builder {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
        };
        let actual = Builder::new(Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_success() {
        let expected = "Hello World";
        let actual = Builder::new(Code::Ok).body("Hello World").body;
        assert_eq!(expected, actual)
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = "Hello World";
        let actual = Builder::new(Code::Ok)
            .body("Overwritten")
            .body("Hello World")
            .body;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_create_success() {
        let expected = Response {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
        };
        let actual = Builder::new(Code::Ok).create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_success() {
        let expected = BTreeMap::from([(String::from("Content-Type"), String::from("text/plain"))]);
        let actual = Builder::new(Code::Ok)
            .header("Content-Type", "text/plain")
            .headers;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_overwrite() {
        let expected = BTreeMap::from([(String::from("Content-Type"), String::from("text/plain"))]);
        let actual = Builder::new(Code::Ok)
            .header("Content-Type", "application/json")
            .header("Content-Type", "text/plain")
            .headers;
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_html_success() {
        let expected = Builder::new(Code::Ok)
            .header("Content-Type", "text/html")
            .header("Content-Length", "13")
            .body("<html></html>");
        let actual = Builder::new(Code::Ok).html("<html></html>");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_json_success() {
        let expected = Builder::new(Code::Ok)
            .header("Content-Type", "application/json")
            .header("Content-Length", "3")
            .body("{ }");
        let actual = Builder::new(Code::Ok).json("{ }");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_url_encoded_success() {
        let expected = Builder::new(Code::Ok)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", "9")
            .body("key=value");
        let actual = Builder::new(Code::Ok).url_encoded("key=value");
        assert_eq!(expected, actual);
    }

    // impl Response

    #[test]
    fn response_new_success() {
        let expected = Response {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
        };
        let actual = Response::new(Code::Ok, BTreeMap::new(), String::new());
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_body_success() {
        let expected = "Hello World";
        let response = Response::build(Code::Ok).body("Hello World").create();
        let actual = response.body();
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_build_success() {
        let expected = Builder {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
        };
        let actual = Response::build(Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_code_success() {
        let expected = Code::Ok;
        let response = Response::build(Code::Ok).create();
        let actual = response.code();
        assert_eq!(expected, *actual);
    }

    #[test]
    fn response_header_success() {
        let expected = Some("text/plain");
        let response = Response::build(Code::Ok)
            .header("Content-Type", "text/plain")
            .create();
        let actual = response.header("Content-Type");
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_header_missing() {
        let expected = None;
        let response = Response::build(Code::Ok)
            .header("Content-Type", "text/plain")
            .create();
        let actual = response.header("Content-Length");
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_headers_success() {
        let expected = BTreeMap::from([(String::from("Content-Type"), String::from("text/plain"))]);
        let response = Response::build(Code::Ok)
            .header("Content-Type", "text/plain")
            .create();
        let actual = response.headers();
        assert_eq!(expected, *actual);
    }

    // impl Display for Response

    #[test]
    fn request_fmt_success() {
        let expected = "\
        HTTP/1.1 200 OK\n\
        Content-Length: 11\n\
        Content-Type: text/plain\n\n\
        Hello World";

        let actual = Response::build(Code::Ok)
            .header("Content-Type", "text/plain")
            .header("Content-Length", "11")
            .body("Hello World")
            .create()
            .to_string();

        assert_eq!(expected, actual);
    }

    // impl Display for Code

    #[test]
    fn version_fmt_default() {
        let expected = "404 Not Found";
        let actual = Code::NotFound.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn version_fmt_ok() {
        let expected = "200 OK";
        let actual = Code::Ok.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn version_fmt_non_authoritative_information() {
        let expected = "203 Non-Authoritative Information";
        let actual = Code::NonAuthoritativeInformation.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn version_fmt_multi_status() {
        let expected = "207 Multi-Status";
        let actual = Code::MultiStatus.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn version_fmt_im_a_teapot() {
        let expected = "418 I'm a teapot";
        let actual = Code::ImATeapot.to_string();
        assert_eq!(expected, actual);
    }
}
