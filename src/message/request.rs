use {
    super::{
        Method,
        Request,
        Resource,
    },
    crate::{
        debug,
        utils::{
            process_header_line,
            process_req_line,
            AppErr,
            AppResult,
        },
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

impl TryFrom<&Vec<u8>> for Request {
    type Error = AppErr;

    fn try_from(req_buf: &Vec<u8>) -> AppResult<Self> {
        let req_str = String::from_utf8_lossy(&req_buf[..]).to_string();

        let mut resource = Resource::Path("".to_string());
        let mut method = Method::Uninitialized;
        let mut headers = HashMap::new();
        let mut body = String::new();

        let mut lines = req_str.lines();
        if let Some(first_line) = lines.next() {
            if first_line.contains("HTTP") {
                let (parsed_method, parsed_resource) = process_req_line(first_line);
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

            if reached_body {
                continue;
            }

            if line.contains(":") {
                let (key, value) = process_header_line(line);
                if key == "Content-Length" {
                    if let Ok(content_length) = value.parse::<usize>() {
                        if debug!(req_buf.len()) < debug!(content_length) {
                            return Err(AppErr::IncompleteRequest);
                        }
                    }
                }
                headers.insert(key, value);
                continue;
            }

            // Append to body (with new line if not first line)
            if !body.is_empty() {
                body.push('\n');
            }

            body.push_str(line);
        }
        // Parse the incoming HTTP request into HttpRequest struct
        Ok(Request {
            resource,
            method,
            headers,
            body: debug!(body),
        })
    }
}
