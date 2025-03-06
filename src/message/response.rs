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

/// Set the default value of the `Response` that
/// will be equivalent to a successful response.
impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: 200,
            status_txt:  String::from("OK"),
            headers:     None,
            body:        None,
        }
    }
}

/// Converts the `Response into a `String` when
/// sending it.
impl From<Response> for String
where
    Response: Sized,
{
    fn from(res: Response) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.status_code,
            &res.status_txt,
            &res.headers(),
            &res.body().len(),
            &res.body()
        )
    }
}

/// Converts
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
            status_txt:  err.message,
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
        response.status_txt = "OK".to_string();
        response.body = body;

        response
    }

    pub fn send(&self, write_stream: &mut impl Write) {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
    }

    pub fn set_status_code(&mut self, status_code: u16) { self.status_code = status_code; }

    pub fn set_status_txt(&mut self, status_txt: String) { self.status_txt = status_txt; }

    pub fn headers(&self) -> String {
        match &self.headers {
            Some(h) => {
                let mut header_string: String = "".into();

                for (k, v) in h.iter() {
                    header_string = format!("{}{}:{}\r\n", header_string, k, v);
                }

                header_string
            }
            None => "".into(),
        }
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}
