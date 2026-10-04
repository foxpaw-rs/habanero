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
//! * [`Url`], representing a URL; and
//! * [`Version`], representing the HTTP version.
//!
//! # Examples
//! Todo: Populate once all types are completed

use std::str::FromStr;

/// `Headers`
///
/// A map of HTTP headers, used by [`Request`] and [`Response`]. Headers are
/// retrieved case-insensitively and stored as provided.
///
/// The `Headers` supports duplicate keys and does not overwrite the previous
/// value for a given key. This is particularly useful in the case of setting
/// and receiving HTTP Cookies for example.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::Headers;
///
/// let mut headers = Headers::new();
///
/// // Insert some values
/// headers.insert("Content-Type", "text/html");
/// headers.insert("Content-Length", "80");
/// headers.insert("Custom-Header", "a");
/// headers.insert("Custom-Header", "b");
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
    /// [`Response`] structs will create their own `Headers` and it is highly
    /// unlikely that as a consumer of the crate you will need to create one.
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
    /// first occurrence is determined by order of insertion within the
    /// `Headers`. Finding is a case-insensitive operation.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let mut headers = Headers::new();
    /// headers.insert("my-value", "a");
    /// headers.insert("my-value", "b");
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
    /// let mut headers = Headers::new();
    /// headers.insert("my-value", "a");
    /// headers.insert("my-value", "b");
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
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Headers;
    ///
    /// let mut headers = Headers::new();
    /// headers.insert("my-value", "a");
    /// headers.insert("my-value", "b");
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.push((key.into(), value.into()));
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
    /// Method will error with a `String` type if the supplied string is not a
    /// valid HTTP URL, or an unsupported [`Scheme`] is specified.
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
    /// assert!(Url::from_str("https://rust-lang.org").is_err());
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
    fn header_map_new_correct() {
        let expected = Headers { inner: Vec::new() };
        let actual = Headers::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_find_correct() {
        let mut headers = Headers::new();
        headers.insert("my-value", "a");

        let expected = Some("a");
        let actual = headers.find("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_find_first() {
        let mut headers = Headers::new();
        headers.insert("my-value", "a");
        headers.insert("my-value", "b");

        let expected = Some("a");
        let actual = headers.find("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_find_case_insensitive() {
        let mut headers = Headers::new();
        headers.insert("my-value", "a");

        let expected = Some("a");
        let actual = headers.find("MY-VALUE");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_find_missing() {
        let mut headers = Headers::new();

        headers.insert("my-value", "a");
        headers.insert("my-value", "b");

        let expected = None;
        let actual = headers.find("not-here");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_get_correct() {
        let mut headers = Headers::new();
        headers.insert("my-value", "a");

        let expected = vec!["a"];
        let actual = headers.get("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_get_all() {
        let mut headers = Headers::new();
        headers.insert("my-value", "a");
        headers.insert("my-value", "b");

        let expected = vec!["a", "b"];
        let actual = headers.get("my-value");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_get_case_insensitive() {
        let mut headers = Headers::new();
        headers.insert("my-value", "a");
        headers.insert("My-Value", "b");

        let expected = vec!["a", "b"];
        let actual = headers.get("MY-VALUE");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_get_none() {
        let mut headers = Headers::new();

        headers.insert("my-value", "a");
        headers.insert("my-value", "b");

        let expected = Vec::<&str>::new();
        let actual = headers.get("not-here");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_insert_correct() {
        let expected = Headers {
            inner: vec![("My-Value".to_string(), "a".to_string())],
        };
        let mut actual = Headers::new();
        actual.insert("My-Value", "a");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_insert_duplicate() {
        let expected = Headers {
            inner: vec![
                ("My-Value".to_string(), "a".to_string()),
                ("My-Value".to_string(), "b".to_string()),
            ],
        };
        let mut actual = Headers::new();
        actual.insert("My-Value", "a");
        actual.insert("My-Value", "b");

        assert_eq!(actual, expected);
    }

    #[test]
    fn header_map_iter_correct() {
        let mut headers = Headers::new();
        headers.insert("My-Value", "a");
        headers.insert("My-Value", "b");
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
