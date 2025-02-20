use {
    super::{
        Data,
        Handler,
    },
    crate::{
        message::{
            Request,
            Resource,
            Response,
        },
        utils::{
            AppErr,
            AppResult,
        },
    },
    std::{
        collections::HashMap,
        env,
        fs,
    },
};

pub struct Api;

impl Handler for Api {
    fn handle(req: &Request) -> AppResult<Response> {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        if route.len() < 3 {
            return Err(AppErr::new("Route length is too short"));
        }

        match route[2] {
            "shipping" if route.len() > 3 && route[3] == "data" => {
                let body =
                    Some(serde_json::to_string(&Self::load_json())?);

                dbg!(&body);

                let mut headers = HashMap::new();
                headers.insert("Content-Type", "applicaion/json");
                Ok(Response::new("200", Some(headers), body))
            }
            _ => Ok(Response::new(
                "404",
                None,
                Self::load_file("error.html"),
            )),
        }
    }
}

impl Api {
    fn load_json() -> Vec<Data> {
        let default_path =
            format!("{}/public/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{data_path}/data.json");
        let json_contents = fs::read_to_string(full_path).unwrap();
        let data = serde_json::from_str(json_contents.as_str()).unwrap();

        data
    }
}
