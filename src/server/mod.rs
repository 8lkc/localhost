mod getters;
mod handler;
mod router;
mod run;
mod middleware;

pub(super) use router::Router;
use crate::Request;

use {
    router::Route,
    serde::{
        Deserialize,
        Serialize,
    },
    std::collections::HashMap,
};

#[derive(Serialize, Deserialize)]
pub struct Server {
    host:             Option<String>,
    ports:            Option<Vec<usize>>,
    root:             Option<String>,
    error_pages:      Option<Vec<String>>,
    uploads_max_size: Option<u64>,
    cgi:              Option<HashMap<String, String>>,
    listing:          Option<bool>,
    routes:           Option<Vec<Route>>,
}

pub struct Middleware<'a> {
    request: &'a Request,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    id:     i32,
    data:   String,
    status: String,
}
