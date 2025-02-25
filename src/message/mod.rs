pub(crate) mod request;
mod response;

use std::collections::HashMap;

pub(super) type Headers = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct Request {
    pub method:   Method,
    pub resource: Resource,
    pub headers:  HashMap<String, String>,
    pub msg_body: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    status_code: u16,
    status_text: String,
    headers:     Option<Headers>,
    body:        Option<String>,
}
