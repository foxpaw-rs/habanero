use crate::transport::Transport;
use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr};
use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockTransport {
    closed: bool,
    read_cursor: Cursor<Vec<u8>>,
    write_buffer: Vec<u8>,
}

impl MockTransport {
    pub fn new(expected: &[u8]) -> Self {
        Self {
            closed: false,
            read_cursor: Cursor::new(expected.to_vec()),
            write_buffer: Vec::new(),
        }
    }

    pub fn close_connection(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn write_buffer(&self) -> &[u8] {
        &self.write_buffer
    }
}

impl Read for MockTransport {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_cursor.read(buf)
    }
}

impl Transport for MockTransport {
    fn set_read_timeout(&mut self, _duration: Option<Duration>) -> io::Result<()> {
        Ok(())
    }

    fn set_write_timeout(&mut self, _duration: Option<Duration>) -> io::Result<()> {
        Ok(())
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:8080"
            .parse()
            .map_err(|_| io::Error::from(ErrorKind::InvalidInput))?)
    }

    fn shutdown(&mut self, _how: Shutdown) -> io::Result<()> {
        Ok(())
    }
}

impl Write for MockTransport {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.closed {
            return Ok(0);
        }
        self.write_buffer.write(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
