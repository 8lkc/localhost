mod api;
mod cgi;
mod http;

use {
    crate::{
        message::{
            Request,
            Response,
        },
        utils::AppResult,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        env,
        fs,
    },
};
pub use {
    api::Api,
    cgi::Cgi,
    http::Http,
};

pub trait Handler {
    fn handle(req: &Request) -> AppResult<Response>;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!(
            "{}/public/templates",
            env!("CARGO_MANIFEST_DIR")
        );
        let templates_path =
            env::var("PUBLIC_VAR").unwrap_or(default_path);
        let full_path = format!("{templates_path}/{file_name}");
        let contents = fs::read_to_string(full_path);

        contents.ok()
    }
}

pub struct ErrorPage;

impl Handler for ErrorPage {
    fn handle(_req: &Request) -> AppResult<Response> {
        Ok(Response::new(
            "404",
            None,
            Self::load_file("error.html"),
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    id:     i32,
    data:   String,
    status: String,
}
