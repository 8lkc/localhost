use {
    super::{
        Method,
        Request,
        Resource,
    },
    crate::utils::{
        process_header_line,
        process_req_line,
    },
    std::{
        collections::HashMap,
        fmt,
    },
};

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => Self::Uninitialized,
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::DELETE => writeln!(f, "DELETE"),
            Method::Uninitialized => write!(f, "Uninitialized Method"),
        }
    }
}

impl Resource {
    pub fn path(&self) -> &str {
        match self {
            Resource::Path(path) => path.as_str(),
        }
    }
}

impl From<String> for Request {
    fn from(req: String) -> Self {
        let mut resource = Resource::Path("".to_string());
        let mut method = Method::Uninitialized;
        let mut headers = HashMap::new();
        let mut body = String::new();

        let mut lines = req.lines();
        if let Some(first_line) = lines.next() {
            if first_line.contains("HTTP") {
                let (parsed_method, parsed_resource) =
                    process_req_line(first_line);
                method = parsed_method;
                resource = parsed_resource;
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
                    headers.insert(key, value);
                }
                else {
                    // Append to body (with new line if not first
                    // line)
                    if !body.is_empty() {
                        body.push('\n');
                    }
                    body.push_str(line);
                }
            }
        }
        // Parse the incoming HTTP request into HttpRequest struct
        Request {
            resource,
            method,
            headers,
            body,
        }
    }
}
