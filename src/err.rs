//! Error
//!
//! This module probides the error types utilised by the crate, along with
//! assistance implementations to convert between error types when relevant.
//!
//! # Architecture
//! The error types defined for this crate are:
//! * [`ParseError`]
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
}
