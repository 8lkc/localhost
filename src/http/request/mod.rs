mod method;
mod utils;

pub use method::{
    Method,
    Resource,
    Version,
};
use {
    std::collections::HashMap,
    utils::{
        process_header_line,
        process_req_line,
    },
};

#[derive(Debug)]
pub struct Request {
    pub method:   Method,
    pub version:  Version,
    pub resource: Resource,
    pub headers:  HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for Request {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        // Read each line in the incoming HTTP request
        for line in req.lines() {
            // If the line read is request line, call function process_req_line();
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            // If the line read is header line, call function
            // process_header_line();
            }
            else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            // If it is blank line, do nothing
            }
            else if line.len() == 0 {
                // If none of these, treat it as message body
            }
            else {
                parsed_msg_body = line;
            }
        }
        // Parse the incoming HTTP request into HttpRequest struct
        Request {
            method:   parsed_method,
            version:  parsed_version,
            resource: parsed_resource,
            headers:  parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}
