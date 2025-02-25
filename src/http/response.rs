use std::collections::HashMap;

pub struct HttpResponse {
    version: String,
    status_code: u16,
    reason_phrase: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
} impl HttpResponse {
    // Creates a new HTTP response with the given status code and reason phrase.
    pub fn new(status_code: u16, reason_phrase: &str) -> Self { HttpResponse {
        version: "HTTP/1.1".to_string(),
        status_code,
        reason_phrase: reason_phrase.to_string(),
        headers: HashMap::new(),
        body: None,
    }}

    // Adds or updates a header.
    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    // Sets the response body.
    pub fn set_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    // Converts the response into a vector of bytes ready to be sent.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("{} {} {}\r\n", self.version, self.status_code, self.reason_phrase);
        
        // Automatically set Content-Length if there's a body and no explicit header.
        if let Some(body) = &self.body {
            if !self.headers.contains_key("Content-Length") {
                response.push_str(&format!("Content-Length: {}\r\n", body.len()));
            }
        }
        
        // Append all headers.
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        // End headers section.
        response.push_str("\r\n");
        
        let mut response_bytes = response.into_bytes();
        if let Some(body) = &self.body { response_bytes.extend_from_slice(body) }
        response_bytes
    }
}
