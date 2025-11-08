//! HTTP Versions.
//!
//! Module to house the supported `Version` enumerated type, providing the
//! currently supported HTTP versions.

use core::fmt::{self, Display, Formatter};

/// The supported HTTP versions.
///
/// Representation of the supported HTTP versions, which are sent via the HTTP
/// request or response.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Version {
    Http1_1,
}

impl Display for Version {
    /// Format the `Version`.
    ///
    /// Formats the `Version` into what would be expected for an HTTP request.
    ///
    /// # Examples
    /// ```rust
    /// use habanero::http::Version;
    /// // or use habanero::request::Version;
    /// // or use habanero::response::Version;
    ///
    /// let version = Version::Http1_1;
    /// let string = version.to_string();
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Version::Http1_1 => "HTTP/1.1",
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // impl Display for Version

    #[test]
    fn version_fmt_success() {
        let expected = "HTTP/1.1";
        let actual = Version::Http1_1.to_string();
        assert_eq!(expected, actual);
    }
}
