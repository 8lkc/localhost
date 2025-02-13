mod method;
pub use method::{
     Method,
     Resource,
};
use {
     crate::utils::{
          process_header_line,
          process_req_line,
     },
     std::collections::HashMap,
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
          let mut parsed_msg_body = String::new();

          let mut lines = req.lines();
          if let Some(first_line) = lines.next() {
               if first_line.contains("HTTP") {
                    let (method, resource) = process_req_line(first_line);
                    parsed_method = method;
                    parsed_resource = resource;
               }
          }

          // Parse headers until empty line
          let mut reached_body = false;
          for line in lines {
               if line.is_empty() {
                    reached_body = true;
                    continue;
               }

               if !reached_body {
                    if line.contains(":") {
                         let (key, value) = process_header_line(line);
                         parsed_headers.insert(key, value);
                    }
                    else {
                         // Append to body (with new line if not first
                         // line)
                         if !parsed_msg_body.is_empty() {
                              parsed_msg_body.push('\n');
                         }
                         parsed_msg_body.push_str(line);
                    }
               }
          }
          // Parse the incoming HTTP request into HttpRequest struct
          Request {
               method:   parsed_method,
               resource: parsed_resource,
               headers:  parsed_headers,
               msg_body: parsed_msg_body,
          }
     }
}
