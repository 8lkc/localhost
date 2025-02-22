mod func;

use {
    crate::{
        server::{
            Handler,
            Http,
        },
        utils::HttpErr,
    },
    std::collections::HashMap,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Response<'a> {
    status_code: u16,
    status_text: String,
    headers:     Option<HashMap<&'a str, &'a str>>,
    body:        Option<String>,
}

impl Default for Response<'_> {
    fn default() -> Self {
        Self {
            status_code: 200,
            status_text: String::from("OK"),
            headers:     None,
            body:        None,
        }
    }
}

impl From<Response<'_>> for String {
    fn from(res: Response) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.status_code(),
            &res.status_text(),
            &res.headers(),
            &res.body().len(),
            &res.body()
        )
    }
}

impl From<HttpErr> for Response<'_> {
    fn from(err: HttpErr) -> Self {
        Self {
            status_code: err.status_code,
            status_text: err.message,
            headers:     Some(HashMap::from([(
                "Content-Type",
                "text/html",
            )])),
            body:        Http::load_file("error.html"),
        }
    }
}
