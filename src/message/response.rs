use {
    super::{
        Headers,
        Response,
    },
    crate::utils::{
        HttpErr,
        TEMPLATES,
    },
    std::{
        collections::HashMap,
        io::Write,
    },
    tera::Context,
};

impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: 200,
            status_text: String::from("OK"),
            headers:     None,
            body:        None,
        }
    }
}

impl From<Response> for String {
    fn from(res: Response) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.status_code,
            &res.status_text,
            &res.headers(),
            &res.body().len(),
            &res.body()
        )
    }
}

impl From<HttpErr> for Response {
    fn from(err: HttpErr) -> Self {
        let mut ctx = Context::new();
        ctx.insert("status_code", &err.status_code);
        ctx.insert("status_text", &err.message);

        let page = TEMPLATES
            .render("error.html", &ctx)
            .unwrap_or_else(|_| {
                format!(
                    "<h1>Error {}</h1><p>{}</p>",
                    err.status_code, err.message
                )
            });

        Self {
            status_code: err.status_code,
            status_text: err.message,
            headers:     Some(HashMap::from([(
                "Content-Type".to_string(),
                "text/html".to_string(),
            )])),
            body:        Some(page),
        }
    }
}

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
