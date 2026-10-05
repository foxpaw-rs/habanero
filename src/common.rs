//! Common
//!
//! This module provides the common data types. Essentially housing the
//! [`Request`] and [`Response`] and associated sub-types used in the crate.
//!
//! # Architecture
//! This module has two primary components:
//! * [`Request`], representing an HTTP request; and
//! * [`Response`], representing an HTTP response.
//!
//! Additionally, a number of smaller components are provided as parts of the
//! [`Request`] and/or [`Response`]:
//! * [`Method`], an enumeration of the supported HTTP methods;
//! * [`Headers`], representing the HTTP headers;
//! * [`Scheme`], an enumeration of the supported URL schemes;
//! * [`Url`], representing a URL; and
//! * [`Version`], representing the HTTP version.
//!
//! # Examples
//! ## Creating a [`Request`]
//! ```rust
//! use foxpaw_habanero::*;
//!
//! fn create_request() -> Result<Request, String> {
//!     let request = Request::new(
//!             Method::Get, "http://rust-lang.org".parse()?, Version::Http11
//!         )
//!         .with_header("my-value", "My Header")
//!         .with_body("Hello World");
//!     Ok(request)
//! }
//! ```
//!
//! ## Creating a [`Response`]
//! ```rust
//! use foxpaw_habanero::*;
//!
//! fn create_response() -> Result<Response, String> {
//!     let response = Response::new(Version::Http11, 200)
//!         .html("<html></html>")
//!         .with_header("my-value", "My Header")
//!         .with_body("Hello World");
//!     Ok(response)
//! }
//! ```

pub use std::str::FromStr;

/// `Headers`
///
/// A map of HTTP headers, used by [`Request`] and [`Response`]. Headers are
/// retrieved case-insensitively and stored as provided.
///
/// `Headers` supports duplicate keys and does not overwrite the previous
/// value for a given key. This is particularly useful in the case of setting
/// and receiving HTTP Cookies for example.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::Headers;
///
/// let headers = Headers::new()
///     .insert("Content-Type", "text/html")
///     .insert("Content-Length", "80")
///     .insert("Custom-Header", "a")
///     .insert("Custom-Header", "b");
///
/// // Retrieve a single value
/// assert_eq!(headers.find("Content-Type"), Some("text/html"));
/// assert_eq!(headers.find("CONTENT-TYPE"), Some("text/html"));
///
/// // If there are multiple values, find will retrieve the first value
/// assert_eq!(headers.find("Custom-Header"), Some("a"));
///
/// // If you have multiple values for a given key, use the get method
/// assert_eq!(headers.get("Custom-Header"), vec!["a", "b"]);
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Headers {
    inner: Vec<(String, String)>,
}

impl Headers {
    /// New
    ///
    /// Create a new empty `Headers` instance. Note that the [`Request`] and
    /// [`Response`] structs will create their own `Headers` and it is unusual
    /// that as a consumer of the crate you will need to create one.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let headers = Headers::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Find
    ///
    /// Find the first occurrence of a header within the `Headers`. The
    /// first occurrence is determined by order of insertion. Find is a
    /// case-insensitive operation.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let headers = Headers::new()
    ///     .insert("my-value", "a")
    ///     .insert("my-value", "b");
    ///
    /// assert_eq!(headers.find("my-value"), Some("a"));
    /// ```
    #[must_use]
    pub fn find(&self, key: &str) -> Option<&str> {
        self.iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }

    /// Get
    ///
    /// Get all occurrences of a header within the `Headers`. If multiple
    /// values exist for the specified key, all will be returned. Get is a
    /// case-insensitive operation.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let headers = Headers::new()
    ///     .insert("my-value", "a")
    ///     .insert("my-value", "b");
    ///
    /// assert_eq!(headers.get("my-value"), vec!["a", "b"]);
    /// ```
    #[must_use]
    pub fn get(&self, key: &str) -> Vec<&str> {
        self.iter()
            .filter(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
            .collect()
    }

    /// Insert
    ///
    /// Insert a key, value pair into the `Headers`. This operation is
    /// non-destructive and will not overwrite a previously inserted value for
    /// the provided key. The inserted key will be entered in exactly as
    /// provided, and only retrieved case-insensitively.
    ///
    /// Note that this method consumes and returns the altered `Headers`
    /// instance and is deigned to be chained, invalidating the original
    /// object.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let headers = Headers::new()
    ///     .insert("my-value", "a")
    ///     .insert("my-value", "b");
    /// ```
    #[must_use]
    pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.push((key.into(), value.into()));
        self
    }

    /// Iter
    ///
    /// Obtain an iterator for the `Headers`.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let headers = Headers::new();
    /// headers.iter().for_each(|header| {
    ///     // Do something...
    /// })
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.inner.iter()
    }
}

/// `Method`
///
/// Enumerated HTTP Methods supported by the crate.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
}

impl FromStr for Method {
    type Err = String;

    /// From Str
    ///
    /// Try to convert from a `str` into a [`Method`]. Can be used explicitly or
    /// implicitly from `str::parse`. Note that while HTTP methods are expected
    /// to be uppercase, this method is case-insensitive.
    ///
    /// # Errors
    /// Method will error with a `String` type if the supplied string is not a
    /// valid HTTP method.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Method;
    /// use std::str::FromStr;
    ///
    /// let expected = Ok(Method::Get);
    ///
    /// // Explicit calling from_str
    /// assert_eq!(Method::from_str("get"), expected);
    ///
    /// // Implicitly via str::parse
    /// assert_eq!("GET".parse(), expected);
    ///
    /// // Invalid input string
    /// assert!(Method::from_str("Unknown").is_err());
    /// assert!("Unknown".parse::<Method>().is_err());
    /// ```
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from.to_uppercase().as_str() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            "OPTIONS" => Ok(Method::Options),
            other => Err(format!("Unsupported HTTP method: {other}")),
        }
    }
}

/// `Request`
///
/// An HTTP request, used both when receiving a HTTP request via a [`Server`]
/// or prepared to be sent via a [`Client`].
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::*;
///
/// fn send_request() -> Result<(), String> {
///     let mut request = Request::new(Method::Get, "http://rust-lang.org".parse()?, Version::Http11);
///
///     // Do things with the request...
///     request.body = "Hello World".into();
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    /// The HTTP method
    pub method: Method,

    /// The URL endpoint
    pub url: Url,

    /// The HTTP version
    pub version: Version,

    /// The headers supplied to the HTTP request
    pub headers: Headers,

    /// The body of the HTTP request
    pub body: Vec<u8>,
}

impl Request {
    /// New
    ///
    /// Create a new empty `Request` instance with the provided HTTP method,
    /// Url and HTTP version.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_request() -> Result<Request, String> {
    ///     let request = Request::new(
    ///         Method::Get,
    ///         "http://rust-lang.org".parse()?,
    ///         Version::Http11
    ///     );
    ///     Ok(request)
    /// }
    /// ```
    #[must_use]
    pub fn new(method: Method, url: Url, version: Version) -> Self {
        Self {
            method,
            url,
            version,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// With Body
    ///
    /// Fluently sets a body on this `Request`, consuming the original object
    /// and returning the updated instance.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_request() -> Result<Request, String> {
    ///     let request = Request::new(
    ///             Method::Get,
    ///             "http://rust-lang.org".parse()?,
    ///             Version::Http11
    ///         )
    ///         .with_body("Hello World");
    ///     Ok(request)
    /// }
    /// ```
    #[must_use]
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// With Header
    ///
    /// Fluently sets a header on this `Request`, consuming the original object
    /// and returning the updated instance.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_request() -> Result<Request, String> {
    ///     let request = Request::new(
    ///             Method::Get,
    ///             "http://rust-lang.org".parse()?,
    ///             Version::Http11
    ///         )
    ///         .with_header("Content-Type", "application/json");
    ///     Ok(request)
    /// }
    /// ```
    #[must_use]
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers = self.headers.insert(key, value);
        self
    }
}

/// `Response`
///
/// An HTTP response, used both when prepared to be sent via a [`Server`] or
/// receiving a HTTP response via a [`Client`].
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::*;
///
/// fn send_response() -> Result<(), String> {
///     let mut response = Response::new(Version::Http11, 200);
///
///     // Do things with the response...
///     response.body = "Hello World".into();
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    /// The HTTP version
    pub version: Version,

    /// The HTTP status code
    pub status: u16,

    /// The headers supplied to the HTTP response
    pub headers: Headers,

    /// The body of the HTTP response
    pub body: Vec<u8>,
}

impl Response {
    /// New
    ///
    /// Create a new empty `Response` instance with the provided HTTP version
    /// and status code,
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_response() -> Result<Response, String> {
    ///     let response = Response::new(
    ///         Version::Http11,
    ///         200
    ///     );
    ///     Ok(response)
    /// }
    /// ```
    #[must_use]
    pub fn new(version: Version, status: u16) -> Self {
        Self {
            version,
            status,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// Html
    ///
    /// Insert an HTML body into the `Response`. Automatically sets the
    /// Content-Type and Content-Length headers based on the provided body.
    ///
    /// Note that this method consumes and returns the altered `Response`
    /// instance and is deigned to be chained, invalidating the original
    /// object.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_html() -> Result<Response, String> {
    ///     let response = Response::new(
    ///         Version::Http11,
    ///         200
    ///     )
    ///     .html("<html></html>");
    ///     Ok(response)
    /// }
    /// ```
    #[must_use]
    pub fn html(mut self, body: impl Into<Vec<u8>>) -> Self {
        let b = body.into();
        self.headers = self
            .headers
            .insert("Content-Type", "text/html")
            .insert("Content-Length", b.len().to_string());
        self.body = b;
        self
    }

    /// Json
    ///
    /// Insert a JSON body into the `Response`. Automatically sets the
    /// Content-Type and Content-Length headers based on the provided body.
    ///
    /// Note that this method consumes and returns the altered `Response`
    /// instance and is deigned to be chained, invalidating the original
    /// object.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_json() -> Result<Response, String> {
    ///     let response = Response::new(
    ///         Version::Http11,
    ///         200
    ///     )
    ///     .json("{value: 42}");
    ///     Ok(response)
    /// }
    /// ```
    #[must_use]
    pub fn json(mut self, body: impl Into<Vec<u8>>) -> Self {
        let b = body.into();
        self.headers = self
            .headers
            .insert("Content-Type", "application/json")
            .insert("Content-Length", b.len().to_string());
        self.body = b;
        self
    }

    /// With Body
    ///
    /// Fluently sets a body on this `Response`, consuming the original object
    /// and returning the updated instance.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_response() -> Result<Response, String> {
    ///     let response = Response::new(Version::Http11, 200)
    ///         .with_body("Hello World");
    ///     Ok(response)
    /// }
    /// ```
    #[must_use]
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// With Header
    ///
    /// Fluently sets a header on this `Response`, consuming the original object
    /// and returning the updated instance.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// fn create_response() -> Result<Response, String> {
    ///     let response = Response::new(Version::Http11, 200)
    ///         .with_header("Content-Type", "application/json");
    ///     Ok(response)
    /// }
    /// ```
    #[must_use]
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers = self.headers.insert(key, value);
        self
    }
}

/// `Scheme`
///
/// Enumerated HTTP Schemes supported by the crate.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scheme {
    Http,
    Https,
}

/// `Url`
///
/// A URL which is used by the [`Client`] as the endpoint to target. This can
/// be created manually, however, the most common and straightforward approach
/// would be to parse a URL from a str using the implemented `FromStr` trait.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::{Url, Scheme};
/// use std::str::FromStr;
///
/// let manual = Url {
///     scheme: Scheme::Http,
///     host: "rust-lang.org".to_string(),
///     port: 80,
///     path: "/".to_string(),
/// };
///
/// let parsed = "http://rust-lang.org".parse();
///
/// assert_eq!(parsed, Ok(manual))
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Url {
    /// The scheme (e.g. http://)
    pub scheme: Scheme,

    /// The host (e.g. rust-lang.org)
    pub host: String,

    /// The port to connect to (e.g. :80)
    pub port: u16,

    /// The remaining path from the url ("/documentation")
    pub path: String,
}

impl FromStr for Url {
    type Err = String;

    /// From Str
    ///
    /// Try to convert from a `str` into a [`Url`]. Can be used explicitly or
    /// implicitly from `str::parse`.
    ///
    /// # Errors
    /// Method will error with a `String` type if the supplied string does not
    /// have a scheme or an unsupported scheme is specified.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::{Scheme, Url};
    /// use std::str::FromStr;
    ///
    /// let expected = Ok(Url {
    ///     scheme: Scheme::Http,
    ///     host: "rust-lang.org".to_string(),
    ///     port: 80,
    ///     path: "/".to_string()
    /// });
    ///
    /// // Explicit calling from_str
    /// assert_eq!(Url::from_str("http://rust-lang.org"), expected);
    ///
    /// // Implicitly via str::parse
    /// assert_eq!("http://rust-lang.org".parse(), expected);
    ///
    /// // Invalid input string
    /// assert!(Url::from_str("Unknown").is_err());
    /// assert!("Unknown".parse::<Url>().is_err());
    ///
    /// // Unsupported URL scheme
    /// assert!(Url::from_str("ftp://rust-lang.org").is_err());
    /// ```
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        let (scheme, rest) = match from.split_once("://") {
            Some((s, r)) if s.eq_ignore_ascii_case("http") => Ok((Scheme::Http, r)),
            Some((s, _)) if s.eq_ignore_ascii_case("https") => {
                Err("Https currently not supported".to_string())
            }
            Some((s, _)) => Err(format!("Unsupported scheme: {s}")),
            None => Err("URL must be in the format [scheme]:://[host][:port?][path]".to_string()),
        }?;

        let (rest, path) = match rest.find('/') {
            Some(idx) => (&rest[..idx], rest[idx..].to_string()),
            None => (rest, "/".to_string()),
        };

        let (host, port) = match rest.split_once(':') {
            Some((h, p)) => (
                h.to_string(),
                p.parse::<u16>()
                    .map_err(|p| format!("Invalid port number: {p}"))?,
            ),
            None => (rest.to_string(), 80),
        };

        Ok(Url {
            scheme,
            host,
            port,
            path,
        })
    }
}

/// `Version`
///
/// Enumerated HTTP Versions supported by the crate.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Version {
    Http10,
    Http11,
    Http20,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_new_correct() {
        let expected = Headers { inner: Vec::new() };
        let actual = Headers::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_find_correct() {
        let headers = Headers::new().insert("my-value", "a");

        let expected = Some("a");
        let actual = headers.find("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_find_first() {
        let headers = Headers::new()
            .insert("my-value", "a")
            .insert("my-value", "b");

        let expected = Some("a");
        let actual = headers.find("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_find_case_insensitive() {
        let headers = Headers::new().insert("my-value", "a");

        let expected = Some("a");
        let actual = headers.find("MY-VALUE");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_find_missing() {
        let headers = Headers::new()
            .insert("my-value", "a")
            .insert("my-value", "b");

        let expected = None;
        let actual = headers.find("not-here");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_get_correct() {
        let headers = Headers::new().insert("my-value", "a");

        let expected = vec!["a"];
        let actual = headers.get("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_get_all() {
        let headers = Headers::new()
            .insert("my-value", "a")
            .insert("my-value", "b");

        let expected = vec!["a", "b"];
        let actual = headers.get("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_get_case_insensitive() {
        let headers = Headers::new()
            .insert("my-value", "a")
            .insert("My-Value", "b");

        let expected = vec!["a", "b"];
        let actual = headers.get("MY-VALUE");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_get_none() {
        let headers = Headers::new()
            .insert("my-value", "a")
            .insert("my-value", "b");

        let expected = Vec::<&str>::new();
        let actual = headers.get("not-here");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_insert_correct() {
        let expected = Headers {
            inner: vec![("My-Value".to_string(), "a".to_string())],
        };
        let actual = Headers::new().insert("My-Value", "a");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_insert_duplicate() {
        let expected = Headers {
            inner: vec![
                ("My-Value".to_string(), "a".to_string()),
                ("My-Value".to_string(), "b".to_string()),
            ],
        };
        let actual = Headers::new()
            .insert("My-Value", "a")
            .insert("My-Value", "b");

        assert_eq!(actual, expected);
    }

    #[test]
    fn headers_iter_correct() {
        let headers = Headers::new()
            .insert("My-Value", "a")
            .insert("My-Value", "b");
        let mut actual = headers.iter();

        assert!(actual.next().is_some());
        assert!(actual.next().is_some());
        assert!(actual.next().is_none());
    }

    #[test]
    fn method_from_str_correct() {
        let expected = Ok(Method::Get);
        let actual = Method::from_str("get");
        assert_eq!(actual, expected);
    }

    #[test]
    fn method_from_str_error() {
        let expected = Err("Unsupported HTTP method: UNKNOWN".to_string());
        let actual = Method::from_str("unknown");
        assert_eq!(actual, expected);
    }

    #[test]
    fn request_new_correct() {
        let expected = Request {
            method: Method::Get,
            url: "http://rust-lang.org".parse().unwrap(),
            version: Version::Http11,
            headers: Headers::new(),
            body: Vec::new(),
        };
        let actual = Request::new(
            Method::Get,
            "http://rust-lang.org".parse().unwrap(),
            Version::Http11,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn request_with_body_correct() {
        let expected = "Hello World".as_bytes();
        let actual = Request::new(
            Method::Get,
            "http://rust-lang.org".parse().unwrap(),
            Version::Http11,
        )
        .with_body("Hello World");
        assert_eq!(actual.body, expected);
    }

    #[test]
    fn request_with_header_correct() {
        let expected = Headers::new().insert("Content-Type", "application/json");
        let actual = Request::new(
            Method::Get,
            "http://rust-lang.org".parse().unwrap(),
            Version::Http11,
        )
        .with_header("Content-Type", "application/json");
        assert_eq!(actual.headers, expected);
    }

    #[test]
    fn response_new_correct() {
        let expected = Response {
            version: Version::Http11,
            status: 200,
            headers: Headers::new(),
            body: Vec::new(),
        };
        let actual = Response::new(Version::Http11, 200);
        assert_eq!(actual, expected);
    }

    #[test]
    fn response_html_correct() {
        let actual = Response::new(Version::Http11, 200).html("<html></html>");
        assert_eq!(actual.headers.find("Content-Type"), Some("text/html"));
        assert_eq!(actual.headers.find("Content-Length"), Some("13"));
        assert_eq!(actual.body, "<html></html>".as_bytes());
    }

    #[test]
    fn response_json_correct() {
        let actual = Response::new(Version::Http11, 200).json("{value: 42}");
        assert_eq!(
            actual.headers.find("Content-Type"),
            Some("application/json")
        );
        assert_eq!(actual.headers.find("Content-Length"), Some("11"));
        assert_eq!(actual.body, "{value: 42}".as_bytes());
    }

    #[test]
    fn response_with_body_correct() {
        let expected = "Hello World".as_bytes();
        let actual = Response::new(Version::Http11, 200).with_body("Hello World");
        assert_eq!(actual.body, expected);
    }

    #[test]
    fn response_with_header_correct() {
        let expected = Headers::new().insert("Content-Type", "application/json");
        let actual =
            Response::new(Version::Http11, 200).with_header("Content-Type", "application/json");
        assert_eq!(actual.headers, expected);
    }

    #[test]
    fn url_from_str_correct() {
        let expected = Url {
            scheme: Scheme::Http,
            host: "rust-lang.org".to_string(),
            port: 80,
            path: "/".to_string(),
        };
        let actual = "http://rust-lang.org".parse();
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn url_from_str_https() {
        // let expected = Url {
        //     scheme: Scheme::Http,
        //     host: "rust-lang.org".to_string(),
        //     port: 80,
        //     path: "/".to_string(),
        // };
        // let actual = "http://rust-lang.org".parse();
        let actual = "https://rust-lang.org".parse::<Url>();
        assert!(actual.is_err());
    }

    #[test]
    fn url_from_str_unsupported_scheme() {
        let expected = "Unsupported scheme: ftp".to_string();
        let actual = "ftp://rust-lang.org".parse::<Url>();
        assert_eq!(actual, Err(expected));
    }

    #[test]
    fn url_from_str_missing_scheme() {
        let expected = "URL must be in the format [scheme]:://[host][:port?][path]".to_string();
        let actual = "rust-lang.org".parse::<Url>();
        assert_eq!(actual, Err(expected));
    }

    #[test]
    fn url_from_str_port() {
        let expected = Url {
            scheme: Scheme::Http,
            host: "rust-lang.org".to_string(),
            port: 8080,
            path: "/".to_string(),
        };
        let actual = "http://rust-lang.org:8080".parse();
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn url_from_str_path() {
        let expected = Url {
            scheme: Scheme::Http,
            host: "rust-lang.org".to_string(),
            port: 80,
            path: "/documentation".to_string(),
        };
        let actual = "http://rust-lang.org/documentation".parse();
        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn url_from_str_port_path() {
        let expected = Url {
            scheme: Scheme::Http,
            host: "rust-lang.org".to_string(),
            port: 8080,
            path: "/documentation".to_string(),
        };
        let actual = "http://rust-lang.org:8080/documentation".parse();
        assert_eq!(actual, Ok(expected));
    }
}
