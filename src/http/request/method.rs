#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}
