//! Codec
//!
//! Todo: Module documentation

use crate::common::{Headers, Method, Request, Response, Version};
use crate::err::ParseError;
use std::io::{self, BufRead, Write};

pub struct Http1;

impl Http1 {
    /// Parse Body
    ///
    /// Parse the body section of the reader. This method will use relevant 
    /// header values in assisting in parsing the HTTP body.
    fn parse_body(reader: &mut impl BufRead, headers: &Headers) -> Result<Vec<u8>, ParseError> {
        let mut body = Vec::new();
        if let Some(len_str) = headers.find("Content-Length") {
            let len: usize = len_str.parse().map_err(|_| {
                ParseError::Invalid(format!("Invalid Content-Length header: {len_str})"))
            })?;
            if len > 0 {
                body.resize(len, 0);
                reader
                    .read_exact(&mut body)
                    .map_err(|e| ParseError::Malformed(e.to_string()))?;
            }
        } else {
            reader
                .read_to_end(&mut body)
                .map_err(|e| ParseError::Malformed(e.to_string()))?;
        }
        Ok(body)
    }

    /// Parse Headers
    ///
    /// Parse the body section of the reader.
    fn parse_headers(reader: &mut impl BufRead) -> Result<Headers, ParseError> {
        let mut headers = Headers::new();
        loop {
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .map_err(|e| ParseError::Malformed(e.to_string()))?;
            line = line.trim_end_matches(['\r', '\n']).to_string();
            if line.is_empty() {
                break;
            }

            match line.split_once(':') {
                Some((k, v)) => headers = headers.insert(k.trim(), v.trim()),
                None => {
                    return Err(ParseError::Malformed(format!(
                        "Malformed header line: {line}"
                    )));
                }
            }
        }
        Ok(headers)
    }

    /// Parse Request
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
    /// use foxpaw_habanero::err::ParseError;
    ///
    /// fn get_request() -> Result<(), ParseError> {
    ///     let input = "GET / HTTP/1.1\n";
    ///     let request = Http1::parse_request(&mut input.as_bytes())?;
    ///     Ok(())
    /// }
    /// ```
    pub fn parse_request(reader: &mut impl BufRead) -> Result<Request, ParseError> {
        let mut line = String::new();
        let bytes = reader
            .read_line(&mut line)
            .map_err(|e| ParseError::Malformed(e.to_string()))?;
        if bytes == 0 {
            return Err(ParseError::Empty(
                "Connection closed, unable to read data".to_string(),
            ));
        }

        let parts: Vec<&str> = line
            .trim_end_matches(['\r', '\n'])
            .split_whitespace()
            .collect();
        if parts.len() != 3 {
            return Err(ParseError::Malformed(
                "Malformed request line, expected: [method] [uri] [version]".to_string(),
            ));
        }

        let method = parts[0].parse()?;
        let url = parts[1];
        let version = parts[2].parse()?;
        if version != Version::Http10 && version != Version::Http11 {
            return Err(ParseError::Invalid(format!(
                "Unsupported HTTP version: {version:?}"
            )));
        }
        let mut request = Request::new(method, url, version);
        request.headers = Self::parse_headers(reader)?;
        request.body = Self::parse_body(reader, &request.headers)?;
        Ok(request)
    }

    /// Parse Response
    ///
    /// Parse a [`Response`] out of a read buffer. The read buffer is expected
    /// to contain an HTTP/1.0 or HTTP/1.1 formatted response.
    ///
    /// # Errors
    /// This method will error in a number of ways due to either a malformed
    /// HTTP response being passed to the method, or an empty response being
    /// passed to the method. This method will return a [`ParseError`]
    /// enumeration with the variant set by the following conditions:
    /// * `ParseError::Empty`: When the response is empty;
    /// * `ParseError::Maformed`: When the response has invalid formatting; or
    /// * `ParseError::Invalid`: When an invalid value is supplied as part of
    ///   the response.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::codec::Http1;
    /// use foxpaw_habanero::err::ParseError;
    ///
    /// fn get_response() -> Result<(), ParseError> {
    ///     let input = "GET / HTTP/1.1\n";
    ///     let response = Http1::parse_response(&mut input.as_bytes())?;
    ///     Ok(())
    /// }
    /// ```
    pub fn parse_response(reader: &mut impl BufRead) -> Result<Response, ParseError> {
        let mut line = String::new();
        let bytes = reader
            .read_line(&mut line)
            .map_err(|e| ParseError::Malformed(e.to_string()))?;
        if bytes == 0 {
            return Err(ParseError::Empty(
                "Connection closed, unable to read data".to_string(),
            ));
        }

        let parts: Vec<&str> = line.trim_end_matches(['\r', '\n']).splitn(3, ' ').collect();
        if parts.len() < 2 {
            return Err(ParseError::Malformed(
                "Malformed response line, expected: [version] [status code] [reason phrase]"
                    .to_string(),
            ));
        }

        let version = parts[0].parse()?;
        if version != Version::Http10 && version != Version::Http11 {
            return Err(ParseError::Invalid(format!(
                "Unsupported HTTP version: {version:?}"
            )));
        }
        let status = parts[1]
            .parse()
            .map_err(|_| ParseError::Invalid(format!("Invalid status code: {}", parts[1])))?;
        let mut response = Response::new(version, status);
        response.headers = Self::parse_headers(reader)?;
        response.body = Self::parse_body(reader, &response.headers)?;
        Ok(response)
    }

    /// Serialise Request
    ///
    /// Serialise a [`Request`] into the provided writer, ready for 
    /// transmission.
    ///
    /// # Errors
    /// This method will error if the underlying calls to write into the writer
    /// fail, returning an `std::io::Error`.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::codec::Http1;
    ///
    /// fn do_serialise_request() -> Result<(), std::io::Error> {
    ///     let request = Request::new(Method::Get, "/", Version::Http11)
    ///         .with_header("Content-Type", "text/plain")
    ///         .with_header("Content-Length", "11")
    ///         .with_body("Hello World");
    ///
    ///     let mut buffer: Vec<u8> = Vec::new(); 
    ///     Http1::serialise_request(&mut buffer, &request)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn serialise_request(
        writer: &mut impl Write,
        request: &Request,
    ) -> Result<(), io::Error> {
        let method = match request.method {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
        };
        let version = match request.version {
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::Http20 => "HTTP/2.0"
        };
        write!(writer, "{method} {} {version}\r\n", request.uri)?;

        let mut has_content_length = false;
        for (k, v) in request.headers.iter() {
            if k.eq_ignore_ascii_case("Content-Length") {
                has_content_length = true;
            }
            write!(writer, "{k}: {v}\r\n")?;
        }

        if !has_content_length && !request.body.is_empty() {
            write!(writer, "Content-Length: {}\r\n", request.body.len())?;
        }
        write!(writer, "\r\n")?;

        if !request.body.is_empty() {
            writer.write_all(&request.body)?;
        }
        writer.flush()?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;

    #[test]
    fn http1_parse_request_correct() {
        let expected = Request::new(Method::Get, "/", Version::Http11);
        let raw = "GET / HTTP/1.1";
        let actual = Http1::parse_request(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_request_correct_headers() {
        let expected = Request::new(Method::Get, "/", Version::Http11)
            .with_header("Content-Type", "application/json");
        let raw = "GET / HTTP/1.1\n\
            Content-Type: application/json";
        let actual = Http1::parse_request(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_request_correct_body() {
        let expected = Request::new(Method::Get, "/", Version::Http11).with_body("Body Text");
        let raw = "GET / HTTP/1.1\n\n\
            Body Text";
        let actual = Http1::parse_request(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_request_correct_headers_body() {
        let expected = Request::new(Method::Get, "/", Version::Http11)
            .with_header("Content-Type", "text/plain")
            .with_body("Body Text");
        let raw = "GET / HTTP/1.1\n\
            Content-Type: text/plain\n\n\
            Body Text";
        let actual = Http1::parse_request(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_request_correct_header_content_length() {
        let expected = Request::new(Method::Get, "/", Version::Http11)
            .with_header("Content-Length", "9")
            .with_body("Body Text");
        let raw = "GET / HTTP/1.1\n\
            Content-Length: 9 \n\n\
            Body Text with some extra";
        let actual = Http1::parse_request(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_request_correct_header_empty_value() {
        let expected =
            Request::new(Method::Get, "/", Version::Http11).with_header("Content-Type", "");
        let raw = "GET / HTTP/1.1\n\
            Content-Type:";
        let actual = Http1::parse_request(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_request_missing_method() {
        let raw = "/ HTTP/1.1";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn http1_parse_request_invalid_method() {
        let raw = "UNKNOWN / HTTP/1.1";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_request_missing_uri() {
        let raw = "GET HTTP/1.1";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn http1_parse_request_missing_version() {
        let raw = "GET /";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn http1_parse_request_invalid_version() {
        let raw = "GET / HTTP/0.1";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_request_invalid_unsupported_version() {
        let raw = "GET / HTTP/2.0";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_request_malformed_header() {
        let raw = "GET / HTTP/1.1\n\
            Content-Type";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn http1_parse_request_invalid_header_content_length() {
        let raw = "GET / HTTP/1.1\n\
            Content-Length: text/plain";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_request_invalid_body_length_content_length() {
        let raw = "GET / HTTP/1.1\n\
            Content-Length: 80\n\n\
            Too short";
        let actual = Http1::parse_request(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn http1_parse_response_correct() {
        let expected = Response::new(Version::Http11, 200);
        let raw = "HTTP/1.1 200 OK";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_correct_missing_reason() {
        let expected = Response::new(Version::Http11, 200);
        let raw = "HTTP/1.1 200";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_correct_headers() {
        let expected =
            Response::new(Version::Http11, 200).with_header("Content-Type", "application/json");
        let raw = "HTTP/1.1 200 OK\n\
            Content-Type: application/json";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_correct_body() {
        let expected = Response::new(Version::Http11, 200).with_body("Body Text");
        let raw = "HTTP/1.1 200 OK\n\n\
            Body Text";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_correct_headers_body() {
        let expected = Response::new(Version::Http11, 200)
            .with_header("Content-Type", "text/plain")
            .with_body("Body Text");
        let raw = "HTTP/1.1 200 OK\n\
            Content-Type: text/plain\n\n\
            Body Text";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_correct_header_content_length() {
        let expected = Response::new(Version::Http11, 200)
            .with_header("Content-Length", "9")
            .with_body("Body Text");
        let raw = "HTTP/1.1 200 OK\n\
            Content-Length: 9 \n\n\
            Body Text with some extra";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_correct_header_empty_value() {
        let expected = Response::new(Version::Http11, 200).with_header("Content-Type", "");
        let raw = "HTTP/1.1 200 OK\n\
            Content-Type:";
        let actual = Http1::parse_response(&mut raw.as_bytes());

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn http1_parse_response_missing_version() {
        let raw = "200 OK";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_response_invalid_version() {
        let raw = "HTTP/2.0 200 OK";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_response_missing_status() {
        let raw = "HTTP/1.1 OK";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_response_invalid_status() {
        let raw = "HTTP/1.1 UNKNOWN OK";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_response_malformed_header() {
        let raw = "HTTP/1.1 200 OK\n\
            Content-Type";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn http1_parse_response_invalid_header_content_length() {
        let raw = "HTTP/1.1 200 OK\n\
            Content-Length: text/plain";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Invalid(_))));
    }

    #[test]
    fn http1_parse_response_invalid_body_length_content_length() {
        let raw = "HTTP/1.1 200 OK\n\
            Content-Length: 80\n\n\
            Too short";
        let actual = Http1::parse_response(&mut raw.as_bytes());
        assert!(matches!(actual, Err(ParseError::Malformed(_))));
    }

    #[test]
    fn serialise_request_correct() {
        let expected = "GET / HTTP/1.1\r\n\r\n".as_bytes();
        let request = Request::new(Method::Get, "/", Version::Http11);
        let mut actual: Vec<u8> = Vec::new();
        Http1::serialise_request(&mut actual, &request).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialise_request_correct_headers() {
        let expected = "GET / HTTP/1.1\r\n\
            Content-Type: text/plain\r\n\r\n".as_bytes();
        let request = Request::new(Method::Get, "/", Version::Http11)
            .with_header("Content-Type", "text/plain");
        let mut actual: Vec<u8> = Vec::new();
        Http1::serialise_request(&mut actual, &request).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialise_request_correct_body() {
        let expected = "GET / HTTP/1.1\r\n\
            Content-Length: 11\r\n\r\n\
            Hello World".as_bytes();
        let request = Request::new(Method::Get, "/", Version::Http11)
            .with_body("Hello World");
        let mut actual: Vec<u8> = Vec::new();
        Http1::serialise_request(&mut actual, &request).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialise_request_correct_headers_body() {
        let expected = "GET / HTTP/1.1\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: 11\r\n\r\n\
            Hello World".as_bytes();
        let request = Request::new(Method::Get, "/", Version::Http11)
            .with_header("Content-Type", "text/plain")
            .with_body("Hello World");
        let mut actual: Vec<u8> = Vec::new();
        Http1::serialise_request(&mut actual, &request).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialise_request_correct_headers_content_length() {
        let expected = "GET / HTTP/1.1\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: 15\r\n\r\n\
            Hello World".as_bytes();
        let request = Request::new(Method::Get, "/", Version::Http11)
            .with_header("Content-Type", "text/plain")
            .with_header("Content-Length", "15")
            .with_body("Hello World");
        let mut actual: Vec<u8> = Vec::new();
        Http1::serialise_request(&mut actual, &request).unwrap();
        assert_eq!(actual, expected);
    }

}
