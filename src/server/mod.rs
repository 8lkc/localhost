mod getters;
mod handler;
mod middleware;
mod router;
mod run;

use {
    crate::Request,
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
    error_pages:      Option<HashMap<String, String>>,
    uploads_max_size: Option<u64>,
    listing:          Option<bool>,
    router:           Option<Router>,
}

#[derive(Serialize, Deserialize)]
pub struct Router {
    routes: Option<Vec<Route>>,
    cgi:    Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
struct Route {
    path:          Option<String>,
    methods:       Option<Vec<String>>,
    default_file:  Option<String>,
    check_session: Option<bool>,
    redirect:      Option<HashMap<String, String>>,
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
