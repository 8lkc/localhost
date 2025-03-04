mod api;
mod cgi;
mod directory_listing;
mod http;

use {
    crate::{
        message::{Request, Response},
        utils::HttpResult,
    },
    directory_listing::FileSystem,
    std::{
        env,
        fs,
    },
};
pub use {api::Api, cgi::Cgi, http::Http, upload::Upload};

pub trait Handler {
    fn handle(req: &Request) -> HttpResult<Response>;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path =
            format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let templates_path =
            env::var("PUBLIC_VAR").unwrap_or(default_path);
        let full_path = format!("{templates_path}/{file_name}");
        let contents = fs::read_to_string(full_path);

        contents.ok()
    }
}
