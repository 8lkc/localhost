mod method;
mod utils;

pub use method::{
    Method,
    Resource,
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
    pub resource: Resource,
    pub headers:  HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for Request {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource) = process_req_line(line);

                parsed_method = method;
                parsed_resource = resource;
            }
            else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            }
            else {
                parsed_msg_body = line;
            }
        }
        // Parse the incoming HTTP request into HttpRequest struct
        Request {
            method:   parsed_method,
            resource: parsed_resource,
            headers:  parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}
