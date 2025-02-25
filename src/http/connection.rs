use std::time::{Duration, Instant};
use mio::net::TcpStream;
use std::io::{self, Read, Write};

pub struct Connection {
    stream: TcpStream,
    last_activity: Instant,
    buffer: Vec<u8>,
    write_buffer: Vec<u8>,
    state: ConnectionState,
} impl Connection {
    pub fn new(stream: TcpStream) -> Self { Self {
        stream,
        last_activity: Instant::now(),
        buffer: Vec::with_capacity(4096),
        write_buffer: Vec::new(),
        state: ConnectionState::Reading,
    }}

    pub fn handle_readable(&mut self) -> io::Result<()> {
        let mut tmp_buffer = [0u8; 4096];
        
        loop {
            match self.stream.read(&mut tmp_buffer) {
                Ok(0) => {
                    self.state = ConnectionState::Closing;
                    return Ok(());
                }
                Ok(n) => {
                    self.last_activity = Instant::now();
                    self.buffer.extend_from_slice(&tmp_buffer[..n]);
                    
                    // Check if we have a complete HTTP request
                    if self.is_complete_request() {
                        self.state = ConnectionState::Writing;
                        self.prepare_response()?;
                        break;
                    }
                }
                Err(output) if output.kind() == io::ErrorKind::WouldBlock => break,
                Err(output) => return Err(output),
            }
        }

        Ok(())
    }

    pub fn handle_writable(&mut self) -> io::Result<()> {
        while !self.write_buffer.is_empty() {
            match self.stream.write(&self.write_buffer) {
                Ok(0) => {
                    self.state = ConnectionState::Closing;
                    return Ok(());
                }
                Ok(n) => {
                    self.last_activity = Instant::now();
                    self.write_buffer.drain(..n);
                }
                Err(output) if output.kind() == io::ErrorKind::WouldBlock => break,
                Err(output) => return Err(output),
            }
        }

        if self.write_buffer.is_empty() {
            self.state = ConnectionState::Reading;
            self.buffer.clear();
        }

        Ok(())
    }

    pub fn is_timed_out(&self, timeout: Duration) -> bool { self.last_activity.elapsed() > timeout }
    pub fn get_state(&self) -> &ConnectionState { &self.state }

    fn is_complete_request(&self) -> bool {
        // Simple check for HTTP request completion
        // In real implementation, this should be more sophisticated
        self.buffer.windows(4).any(|window| window == b"\r\n\r\n")
    }

    fn prepare_response(&mut self) -> io::Result<()> {
        // Basic response for now - this will be improved in HTTP handling phase
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
        self.write_buffer.extend_from_slice(response.as_bytes());
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum ConnectionState {Reading, Writing, Closing}
