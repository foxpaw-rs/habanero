//! Codec
//!
//! Todo: Module documentation
//! Todo: Better error handling across the crate before moving on.

use crate::common::*;
use std::io::{self, BufRead, Write};

pub struct Http1;

impl Http1 {
    /// Parse Request
    ///
    /// Todo: Test once error handling updated
    ///
    /// Parse a [`Request`] out of a read buffer. The read buffer is expected
    /// to contain an HTTP/1.0 or HTTP/1.1 formatted request.
    ///
    /// # Errors
    /// This method will error in a number of ways due to either a malformed
    /// HTTP request being passed to the method, or an empty request being
    /// passed to the method. This method will return a [`ParseError`]
    /// enumeration with the variant set by the following conditions:
    /// * `ParseError::Empty`: When the request is empty;
    /// * `ParseError::Maformed`: When the request has invalid formatting; or
    /// * `ParseError::Invalid`: When an invalid value is supplied as part of
    ///   the request.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::codec::Http1;
    ///
    /// fn get_request() -> Result<(), String> {
    ///     let input = "GET http://rust-lang.org HTTP/1.1\n";
    ///     let request = Http1::parse_request(&mut input.as_bytes());
    ///     Ok(())
    /// }
    /// ```
    pub fn parse_request(reader: &mut impl BufRead) -> Result<Request, String> {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if bytes == 0 {
            return Err("Connection closed, unable to read data".to_string());
        }

        let parts: Vec<&str> = line
            .trim_end_matches(['\r', '\n'])
            .split_whitespace()
            .collect();
        if parts.len() != 3 {
            return Err("Invalid request line, expected: [method] [url] [version]".to_string());
        }

        let method = Method::from_str(parts[0])?;
        let url = parts[1];
        let version = match parts[2] {
            s if s.eq_ignore_ascii_case("HTTP/1.0") => Version::Http10,
            s if s.eq_ignore_ascii_case("HTTP/1.1") => Version::Http11,
            s => return Err(format!("Unsupported HTTP version: {s}")),
        };
        let mut request = Request::new(method, url, version);

        loop {
            reader.read_line(&mut line).map_err(|e| e.to_string())?;
            line = line.trim_end_matches(['\r', '\n']).to_string();
            if line.is_empty() {
                break;
            }

            match line.split_once(':') {
                Some((k, v)) => request = request.with_header(k.trim(), v.trim()),
                None => return Err(format!("Invalid header line: {line}")),
            }
        }

        let mut body = Vec::new();
        if let Some(len_str) = request.headers.find("Content-Length") {
            let len: usize = len_str
                .parse()
                .map_err(|_| format!("Invalid Content-Length header: {len_str})"))?;
            if len > 0 {
                body.resize(len, 0);
                reader.read_exact(&mut body).map_err(|e| e.to_string())?;
            }
        } else {
            reader.read_to_end(&mut body).map_err(|e| e.to_string())?;
        }
        request = request.with_body(body);

        Ok(request)
    }

    /// Parse Response
    ///
    /// Todo: Document / Test
    ///
    /// # Errors
    pub fn parse_response(reader: &mut impl BufRead) -> Result<Request, String> {
        unimplemented!()
    }

    /// Serialise Request
    ///
    /// Todo: Document / Test
    ///
    /// # Errors
    pub fn serialise_request(
        writer: &mut impl Write,
        response: &Response,
    ) -> Result<(), io::Error> {
        unimplemented!()
    }

    /// Serialise Response
    ///
    /// Todo: Document / Test
    ///
    /// # Errors
    pub fn serialise_response(
        writer: &mut impl Write,
        response: &Response,
    ) -> Result<(), io::Error> {
        unimplemented!()
    }
}
