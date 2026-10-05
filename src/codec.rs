//! Codec
//!
//! Todo: Module documentation

use crate::common::*;
use std::io::{self, BufRead, Write};

pub struct Http1;

impl Http1 {
    /// Parse Request
    ///
    /// Todo: Document / Test
    pub fn parse_request(reader: &mut impl BufRead) -> Result<Request, String> {
        unimplemented!()
    }

    /// Parse Response
    ///
    /// Todo: Document / Test
    pub fn parse_response(reader: &mut impl BufRead) -> Result<Request, String> {
        unimplemented!()
    }

    /// Serialise Request
    ///
    /// Todo: Document / Test
    pub fn serialise_request(
        writer: &mut impl Write,
        response: &Response,
    ) -> Result<(), io::Error> {
        unimplemented!()
    }

    /// Serialise Response
    ///
    /// Todo: Document / Test
    pub fn serialise_response(
        writer: &mut impl Write,
        response: &Response,
    ) -> Result<(), io::Error> {
        unimplemented!()
    }
}
