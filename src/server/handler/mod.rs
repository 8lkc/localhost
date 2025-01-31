mod static_page;
mod web_service;

use {
    crate::http::{
        Request,
        Response,
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
    static_page::StaticPage,
    web_service::WebService,
};

pub trait Handler {
    fn handle(req: &Request) -> Result<Response, String>;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path =
            format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_VAR").unwrap_or(default_path);
        let full_path = format!("{public_path}/{file_name}");
        let contents = fs::read_to_string(full_path);

        contents.ok()
    }
}

pub struct ErrorPage;

impl Handler for ErrorPage {
    fn handle(_req: &Request) -> Result<Response, String> {
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
