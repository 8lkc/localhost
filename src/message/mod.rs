pub(crate) mod request;
mod response;
// mod headers;

use std::collections::HashMap;

pub use {
    request::{
        Method,
        Request,
        Resource,
    },
    response::Response,
};

pub(super) type Headers = HashMap<String, String>;
