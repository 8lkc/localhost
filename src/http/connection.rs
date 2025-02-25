use std::time::{Duration, Instant};
use mio::net::TcpStream;
use std::io::{self, Read, Write};

use crate::http::request::{HttpRequest, HttpMethod};
use crate::http::response::HttpResponse;

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
        // A simple check for HTTP request completion by looking for "\r\n\r\n".
        self.buffer.windows(4).any(|window| window == b"\r\n\r\n")
    }

    fn prepare_response(&mut self) -> io::Result<()> {
        match HttpRequest::parse(&self.buffer) {
            Ok(request) => {
                println!("Parsed HTTP request: {:#?}", request);
                let response = match request.get_method() {
                    HttpMethod::GET => {
                        // For GET requests, we respond with a simple message.
                        HttpResponse::new(200, "OK")
                            .set_header("Content-Type", "text/plain")
                            .set_body(b"Hello, World!".to_vec())
                    }
                    _ => {
                        // For any non-GET requests, return a 405 Method Not Allowed.
                        HttpResponse::new(405, "Method Not Allowed")
                            .set_header("Content-Length", "0")
                    }
                };
                self.write_buffer.extend_from_slice(&response.to_bytes());
            }
            Err(err) => {
                println!("Error parsing HTTP request: {:#?}", err);
                let response = HttpResponse::new(400, "Bad Request")
                    .set_header("Content-Length", "0");
                self.write_buffer.extend_from_slice(&response.to_bytes());
            }
        }
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum ConnectionState {Reading, Writing, Closing}
