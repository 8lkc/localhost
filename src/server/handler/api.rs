use {
    super::Handler,
    crate::{
        message::{
            Request,
            Resource,
            Response,
        },
        utils::HttpResult,
    },
    std::{
        collections::HashMap,
        env,
        fs,
    },
};

pub struct Api;

impl Handler for Api {
    fn handle(req: &Request) -> HttpResult<Response> {
        let Resource::Path(path) = &req.resource;

        let default_path = format!(
            "{}/public{}.json",
            env!("CARGO_MANIFEST_DIR"),
            path
        );
        let file_path = env::var("JSON_PATH").unwrap_or(default_path);
        let json_contents = fs::read_to_string(file_path)?;

        Ok(Response::new(
            200,
            Some(HashMap::from([(
                "Content-Type",
                "application/json",
            )])),
            Some(json_contents),
        ))
    }
}
