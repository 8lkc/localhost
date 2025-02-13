mod static_page;
mod web_service;

use {
     crate::{
          utils::Result,
          http::{
               Request,
               Response,
          },
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
     fn handle(req: &Request) -> Result<Response>;
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
     fn handle(_req: &Request) -> Result<Response> {
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
