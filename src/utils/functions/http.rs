use {
    crate::{
        utils::{
            AppErr,
            AppResult,
        },
        Method,
        Resource,
    },
    std::{
        io::{
            BufRead,
            BufReader,
            ErrorKind,
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

    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                req_str.push_str(&line);

                if line == "\r\n" || line == "\n" {
                    break;
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
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
