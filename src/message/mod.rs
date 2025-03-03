pub(crate) mod request;
mod response;

use std::collections::HashMap;

pub(super) type Headers = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    DELETE,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct Request {
    pub resource: Resource,
    pub method:   Method,
    pub headers:  Headers,
    pub body:     String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    status_code: u16,
    status_txt:  String,
    headers:     Option<Headers>,
    body:        Option<String>,
}
