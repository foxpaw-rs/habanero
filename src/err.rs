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
///}
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
/// The enumeration of available serialisation error types. Each variant houses
/// an inner `String` context to be provided by the user of the error.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::err::SerializeError;
///
/// fn my_parsing_method(input: &str) -> Result<(), SerializeError> {
///     if input.is_empty() {
///         return Err(SerializeError::Io("Writing Error.".to_string()));
///     }
///     Ok(())
///}
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
