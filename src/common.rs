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
    /// ````
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
}
