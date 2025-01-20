use super::method::{
    Method,
    Resource,
};

pub(super) fn process_req_line(s: &str) -> (Method, Resource) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
    )
}

pub(super) fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let key = header_items
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    let value = header_items
        .collect::<Vec<&str>>()
        .join(":")
        .trim()
        .to_string();

    (key, value)
}
