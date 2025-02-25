mod func;

use {
    super::Headers,
    crate::utils::{
        HttpErr,
        TEMPLATES,
    },
    std::collections::HashMap,
    tera::Context,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    status_code: u16,
    status_text: String,
    headers:     Option<Headers>,
    body:        Option<String>,
}

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
