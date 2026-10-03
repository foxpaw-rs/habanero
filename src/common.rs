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

/// Method
///
/// Enumerated HTTP Methods supported by the crate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ///    use foxpaw_habanero::Method;
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

/// Version
///
/// Enumerated HTTP Versions supported by the crate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http20,
}

#[cfg(test)]
mod tests {
    use super::*;

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
