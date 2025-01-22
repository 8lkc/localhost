mod func;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Response<'a> {
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self {
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<Response<'a>> for String {
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
