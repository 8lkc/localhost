mod func;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Response<'a> {
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl Default for Response<'_> {
    fn default() -> Self {
        Self {
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
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
