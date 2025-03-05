use {
    super::{
        Api,
        Handler,
    },
    crate::{
        message::{
            Method,
            Request,
            Resource,
            Response,
        },
        server::Middleware,
        utils::HttpResult,
    },
    std::{
        collections::HashMap,
        env,
        fs,
    },
};

impl Handler for Api {
    fn handle(req: &Request) -> HttpResult<Response> {
        Middleware::check(req)
            .logger()?
            .method(Method::GET)?;

        let Resource::Path(path) = &req.resource;

        let default_path = format!(
            "{}/public{}.json",
            env!("CARGO_MANIFEST_DIR"),
            path
        );
        let file_path = env::var("JSON_PATH").unwrap_or(default_path);
        let json_contents = fs::read_to_string(file_path)?;

        Ok(Response::ok(
            Some(HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])),
            Some(json_contents),
        ))
    }
}
