use {
    crate::{
        message::{
            Method,
            Resource,
        },
        utils::{
            AppErr,
            AppResult,
        },
    },
    std::{
        io::{
            BufRead,
            BufReader,
            ErrorKind,
            Read,
        },
        net::TcpStream,
    },
};

pub fn process_req_line(s: &str) -> (Method, Resource) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
    )
}

pub fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let key = header_items
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    let value = header_items
        .collect::<Vec<&str>>()
        .join(":")
        .trim()
        .to_string();

    (key, value)
}

pub fn read_buffer(stream: &TcpStream) -> AppResult<String> {
    let mut buf_reader = BufReader::new(stream);
    let mut req_str = String::new();
    let mut headers_complete = false;
    let mut content_length: usize = 0;

    // First read headers
    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                req_str.push_str(&line);

                // Check for Content-Length header
                if line
                    .to_lowercase()
                    .starts_with("content-length:")
                {
                    if let Some(len_str) = line.split(':').nth(1) {
                        if let Ok(len) = len_str
                            .trim()
                            .parse::<usize>()
                        {
                            content_length = len;
                        }
                    }
                }

                // End of headers
                if line == "\r\n" || line == "\n" {
                    headers_complete = true;
                    break;
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }

    // If we have a content length and headers are complete, read the body
    if headers_complete && content_length > 0 {
        let mut body = vec![0; content_length];
        match buf_reader.read_exact(&mut body) {
            Ok(_) => {
                if let Ok(body_str) = String::from_utf8(body) {
                    req_str.push_str(&body_str);
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                // Try to read at least some data in non-blocking mode
                let mut partial_body = Vec::new();
                let _ = buf_reader.read_to_end(&mut partial_body);
                if let Ok(body_str) = String::from_utf8(partial_body) {
                    req_str.push_str(&body_str);
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    if req_str.is_empty() {
        Err(AppErr::EmptyBuffer)
    }
    else {
        Ok(req_str)
    }
}
