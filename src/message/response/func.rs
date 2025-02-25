use {
    super::Response,
    crate::message::Headers,
    std::{
        collections::HashMap,
        io::Write,
    },
};

impl Response {
    pub fn ok(headers: Option<Headers>, body: Option<String>) -> Response {
        let mut response = Response::default();

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert(
                    "Content-Type".to_string(),
                    "text/html".to_string(),
                );
                Some(h)
            }
        };
        response.status_text = "OK".to_string();
        response.body = body;

        response
    }

    pub fn send(&self, write_stream: &mut impl Write) {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        &self.status_text
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}
