mod getters;
pub mod handler;
mod middleware;
mod router;
mod run;
mod session;
mod validation;

use {
    crate::Request,
    serde::{
        Deserialize,
        Serialize,
    },
    std::{collections::HashMap, time::Duration},
};

#[derive(Serialize, Deserialize)]
pub struct Server {
    host:    Option<String>,
    ports:   Option<Vec<usize>>,
    root:    Option<String>,
    errors:  Option<HashMap<String, String>>,
    uploads: Option<u64>,
    listing: Option<bool>,
    router:  Option<Router>,
}

#[derive(Serialize, Deserialize)]
pub struct Router {
    routes: Option<Vec<Route>>,
    cgi:    Option<HashMap<String, String>>,
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

#[derive(Serialize, Deserialize)]
pub struct Data {
    id:     i32,
    data:   String,
    status: String,
}

pub struct SessionStore {
    pub  sessions: HashMap<String, u64>,
    pub timeout: Duration,
}