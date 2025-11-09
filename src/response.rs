//! HTTP responses.
//! Todo(Paul): Module documentation.

pub use crate::http::Version;
use std::collections::BTreeMap;

/// HTTP Response Builder.
///
/// Utilizes the builder pattern to fluently construct a `Response`. Each
/// method call invalidates the previous `Builder`, and it is intended to be
/// chained from initial construction all the way to the finalize, `create`
/// method to create the `Response`. If multiple `Responses` are required based
/// off the same set of information, the `Builder` should be cloned.
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
//
/// let response = Response::build(Version::Http1_1, Code::Ok)
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Builder {
    body: String,
    code: Code,
    headers: BTreeMap<String, String>,
    version: Version,
}

impl Builder {
    /// Create a new `Builder`.
    ///
    /// Create a new `Builder` via the `Response::build` method to invoke the
    /// builder pattern and build up a `Response`.
    fn new(version: Version, code: Code) -> Self {
        Self {
            body: String::new(),
            code,
            headers: BTreeMap::new(),
            version,
        }
    }

    /// Set a `Response` body.
    ///
    /// Set a HTTP body on the `Response`. This will overwrite any previously
    /// set value for the response body.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // };
    ///
    /// // Note: The final response body will be "{ ... }".
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .body("<html>...</html>")
    ///     .body("{ ... }")
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
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //      Response,
    /// //      response::{
    /// //          Builder, Code, Version
    /// //      }
    /// //  };
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .header("Content-Type", "application/json")
    ///     .body("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn create(self) -> Response {
        Response::new(self.version, self.code, self.headers, self.body)
    }

    /// Set a `Response` header.
    ///
    /// Set a HTTP header on the `Response`. This will overwrite any previously
    /// set value for that header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // };
    ///
    /// // Note: The final "Content-Type" header will be "text/html".
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .header("Content-Type", "application/json")
    ///     .header("Content-Type", "text/html")
    ///     .create();
    /// ```
    #[must_use]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set a `Response` HTML body.
    ///
    /// Set a HTML HTTP body on the `Response`. This will overwrite any
    /// previously set value for the response body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // };
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .html("<html>...</html>")
    ///     .create();
    /// ```
    #[must_use]
    pub fn html(mut self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();

        self.body = body;
        self.header("Content-Type", "text/html")
            .header("Content-Length", len)
    }

    /// Set a `Response` JSON body.
    ///
    /// Set a JSON HTTP body on the `Response`. This will overwrite any
    /// previously set value for the response body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // };
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .json("{ ... }")
    ///     .create();
    /// ```
    #[must_use]
    pub fn json(mut self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();

        self.body = body;
        self.header("Content-Type", "application/json")
            .header("Content-Length", len)
    }

    /// Set a `Response` url encoded body.
    ///
    /// Set a url encoded HTTP body on the `Response`. This will overwrite any
    /// previously set value for the response body, Content-Type header and
    /// Content-Length header.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // };
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .url_encoded("name=MyName&email=test%40test.com")
    ///     .create();
    /// ```
    #[must_use]
    pub fn url_encoded(mut self, body: impl Into<String>) -> Self {
        let body = body.into();
        let len = body.len().to_string();

        self.body = body;
        self.header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", len)
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
/// ```rust
/// use habanero::response::*;
/// // Or use habanero::{
/// //      Response,
/// //      response::{
/// //          Builder, Code, Version
/// //      }
/// //  };
//
/// let response = Response::build(Version::Http1_1, Code::Ok)
///     .header("Content-Type", "application/json")
///     .body("{ ... }")
///     .create();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    body: String,
    code: Code,
    headers: BTreeMap<String, String>,
    version: Version,
}

impl Response {
    /// Create a new `Response`.
    ///
    /// Creates a new response, invoked via the `Builder::create` method to
    /// finalise the construction of the `Response`.
    fn new(version: Version, code: Code, headers: BTreeMap<String, String>, body: String) -> Self {
        Self {
            body,
            code,
            headers,
            version,
        }
    }

    /// Retrieve the Response body
    ///
    /// Retrieve an immutable reference to the body stored in the Response.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // }
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .body("<html>...</html>")
    ///     .create();
    /// let body = response.body();
    /// ```
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

    /// Retrieve the Response code
    ///
    /// Retrieve an immutable reference to the code stored in the Response.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // }
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .create();
    /// let code = response.code();
    /// ```
    pub fn code(&self) -> &Code {
        &self.code
    }

    /// Retrieve the specified Response header
    ///
    /// Retrieve an immutable reference to the specified header stored in the
    /// Response. Will either return the header value or None if the header is
    /// not set.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // }
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .header("Content-Type", "text/html")
    ///     .create();
    /// let header = response.header("Content-Type");
    /// ```
    pub fn header(&self, key: impl Into<String>) -> Option<&str> {
        self.headers.get(&key.into()).map(|value| value.as_str())
    }

    /// Retrieve the Response headers
    ///
    /// Retrieve an immutable reference to the headers stored in the Response.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // }
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .header("Content-Type", "text/html")
    ///     .create();
    /// let headers = response.headers();
    /// ```
    pub fn headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }

    /// Retrieve the Response version
    ///
    /// Retrieve an immutable reference to the version stored in the Response.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::response::*;
    /// // Or use habanero::{
    /// //     Response,
    /// //     response::{Builder, Code, Version}
    /// // }
    ///
    /// let response = Response::build(Version::Http1_1, Code::Ok)
    ///     .create();
    /// let version = response.version();
    /// ```
    pub fn version(&self) -> &Version {
        &self.version
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
        let expected = Builder {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
            version: Version::Http1_1,
        };
        let actual = Builder::new(Version::Http1_1, Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_body_success() {
        let expected = "body";
        let actual = Builder::new(Version::Http1_1, Code::Ok).body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_body_overwrite() {
        let expected = "body";
        let actual = Builder::new(Version::Http1_1, Code::Ok)
            .body("not_body")
            .body("body");

        assert_eq!(expected, actual.body);
    }

    #[test]
    fn builder_create_success() {
        let expected = Response {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
            version: Version::Http1_1,
        };

        let actual = Builder::new(Version::Http1_1, Code::Ok).create();
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_header_success() {
        let mut expected = BTreeMap::new();
        expected.insert("key".to_string(), "value".to_string());

        let actual = Builder::new(Version::Http1_1, Code::Ok).header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_header_overwrite() {
        let mut expected = BTreeMap::new();
        expected.insert("key".to_string(), "value".to_string());

        let actual = Builder::new(Version::Http1_1, Code::Ok)
            .header("key", "not_value")
            .header("key", "value");

        assert_eq!(expected, actual.headers);
    }

    #[test]
    fn builder_html_success() {
        let actual = Builder::new(Version::Http1_1, Code::Ok).html("<html>...</html>");
        let expected = Builder::new(Version::Http1_1, Code::Ok)
            .body("<html>...</html>")
            .header("Content-Type", "text/html")
            .header("Content-Length", "16");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_json_success() {
        let actual = Builder::new(Version::Http1_1, Code::Ok).json("{ ... }");
        let expected = Builder::new(Version::Http1_1, Code::Ok)
            .body("{ ... }")
            .header("Content-Type", "application/json")
            .header("Content-Length", "7");
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_url_encoded_success() {
        let actual = Builder::new(Version::Http1_1, Code::Ok)
            .url_encoded("name=MyName&email=test%40test.com");
        let expected = Builder::new(Version::Http1_1, Code::Ok)
            .body("name=MyName&email=test%40test.com")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", "33");
        assert_eq!(expected, actual);
    }

    // impl Response

    #[test]
    fn response_new_success() {
        let expected = Response {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
            version: Version::Http1_1,
        };
        let actual = Response::new(Version::Http1_1, Code::Ok, BTreeMap::new(), String::new());
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_body_success() {
        let expected = "<html>...</html>";
        let response = Response::build(Version::Http1_1, Code::Ok)
            .body("<html>...</html>")
            .create();
        let actual = response.body();
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_build_success() {
        let expected = Builder {
            body: String::new(),
            code: Code::Ok,
            headers: BTreeMap::new(),
            version: Version::Http1_1,
        };
        let actual = Response::build(Version::Http1_1, Code::Ok);
        assert_eq!(expected, actual);
    }

    #[test]
    fn response_header_success() {
        let expected = Some("text/html");

        let response = Response::build(Version::Http1_1, Code::Ok)
            .header("Content-Type", "text/html")
            .header("Content-Length", "0")
            .create();
        let actual = response.header("Content-Type");

        assert_eq!(expected, actual);
    }

    #[test]
    fn response_header_missing() {
        let expected = None;

        let response = Response::build(Version::Http1_1, Code::Ok).create();
        let actual = response.header("Content-Type");

        assert_eq!(expected, actual);
    }

    #[test]
    fn response_headers_success() {
        let mut expected = BTreeMap::new();
        expected.insert("Content-Type".to_string(), "text/html".to_string());
        expected.insert("Content-Length".to_string(), "0".to_string());

        let response = Response::build(Version::Http1_1, Code::Ok)
            .header("Content-Type", "text/html")
            .header("Content-Length", "0")
            .create();
        let actual = response.headers();

        assert_eq!(expected, *actual);
    }

    #[test]
    fn response_code_success() {
        let expected = Code::Ok;
        let response = Response::build(Version::Http1_1, Code::Ok).create();
        let actual = response.code();
        assert_eq!(expected, *actual);
    }

    #[test]
    fn response_version_success() {
        let expected = Version::Http1_1;
        let response = Response::build(Version::Http1_1, Code::Ok).create();
        let actual = response.version();
        assert_eq!(expected, *actual);
    }
}
