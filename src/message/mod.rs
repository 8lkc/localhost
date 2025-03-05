pub(crate) mod request;
mod response;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub(super) enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub(super) enum Method {
    GET,
    POST,
    DELETE,
    Uninitialized,
}

pub(super) type Headers = HashMap<String, String>;

#[derive(Debug)]
pub(super) struct Request {
    pub resource: Resource,
    pub method:   Method,
    pub headers:  Headers,
    pub body:     String,
}

#[derive(Debug, PartialEq, Clone)]
pub(super) struct Response {
    status_code: u16,
    status_txt:  String,
    headers:     Option<Headers>,
    body:        Option<String>,
}
