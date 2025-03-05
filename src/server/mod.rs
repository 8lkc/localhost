pub mod handler;
mod middleware;
mod router;
mod server;
mod session;

use {
    crate::message::Request,
    handler::{
        Api,
        Cgi,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        collections::HashMap,
        time::Duration,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Server {
    host:    Option<String>,
    ports:   Option<Vec<usize>>,
    root:    Option<String>,
    uploads: Option<u64>,
    listing: Option<bool>,
    router:  Option<Router>,
}

#[derive(Serialize, Deserialize)]
pub struct Router {
    routes:      Option<Vec<Route>>,
    error_pages: Option<HashMap<String, String>>,
    cgi:         Option<Cgi>,
    api:         Option<Api>,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    path:         Option<String>,
    methods:      Option<Vec<String>>,
    default_file: Option<String>,
    session:      Option<bool>,
    redirect:     Option<HashMap<String, String>>,
}

pub(super) struct Middleware<'a> {
    request: &'a Request,
}

pub struct SessionStore {
    pub sessions: HashMap<String, u64>,
    pub timeout:  Duration,
}
