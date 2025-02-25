// src/http/request.rs

use std::collections::HashMap;
use std::str;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Option<HttpMethod> {
        match s {
            "GET"     => Some(HttpMethod::GET),
            "POST"    => Some(HttpMethod::POST),
            "PUT"     => Some(HttpMethod::PUT),
            "DELETE"  => Some(HttpMethod::DELETE),
            "HEAD"    => Some(HttpMethod::HEAD),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "PATCH"   => Some(HttpMethod::PATCH),
            "TRACE"   => Some(HttpMethod::TRACE),
            "CONNECT" => Some(HttpMethod::CONNECT),
            _         => None,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    Utf8Error(std::str::Utf8Error),
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(err: std::str::Utf8Error) -> Self {
        ParseError::Utf8Error(err)
    }
}

impl HttpRequest {
    /// Parse a raw HTTP request from a byte slice.
    pub fn parse(buffer: &[u8]) -> Result<HttpRequest, ParseError> {
        // Convert the byte slice to a string.
        let request_str = str::from_utf8(buffer)?;
        
        // Split into header and (optional) body by the double CRLF.
        let mut parts = request_str.split("\r\n\r\n");
        let header_part = parts.next().ok_or(ParseError::InvalidRequest)?;
        let body_part = parts.next(); // may be None if no body
        
        let mut lines = header_part.lines();

        // Parse the request line (e.g., "GET /index.html HTTP/1.1")
        let request_line = lines.next().ok_or(ParseError::InvalidRequest)?;
        let mut request_line_parts = request_line.split_whitespace();

        let method_str = request_line_parts
            .next()
            .ok_or(ParseError::InvalidRequest)?;
        let path = request_line_parts
            .next()
            .ok_or(ParseError::InvalidRequest)?
            .to_string();
        let version = request_line_parts
            .next()
            .ok_or(ParseError::InvalidRequest)?
            .to_string();

        let method = HttpMethod::from_str(method_str).ok_or(ParseError::InvalidMethod)?;

        // Parse header lines into a HashMap.
        let mut headers = HashMap::new();
        for line in lines {
            if let Some(idx) = line.find(':') {
                let key = line[..idx].trim().to_string();
                let value = line[idx + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Capture the body if available.
        let body = body_part.map(|s| s.as_bytes().to_vec());

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}
