mod api;
mod cgi;
mod file_system;
pub mod http;
mod upload;

use {
    super::SessionStore,
    crate::{
        message::{
            Request,
            Response,
        },
        utils::HttpResult,
    },
    file_system::FileSystem,
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        collections::HashMap,
        env,
        fs,
    },
};

pub trait Handler {
    fn handle(req: &Request) -> HttpResult<Response>;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let templates_path = env::var("PUBLIC_VAR").unwrap_or(default_path);
        let full_path = format!("{templates_path}/{file_name}");
        let contents = fs::read_to_string(full_path);

        contents.ok()
    }
}

pub struct Http {
    pub session_store: SessionStore,
}

type Interpreters = HashMap<String, String>;

/// Common Gateway Interface
#[derive(Serialize, Deserialize)]
pub struct Cgi {
    interpreters: Option<Interpreters>,
}

#[derive(Serialize, Deserialize)]
pub struct Api;

pub struct Upload;
