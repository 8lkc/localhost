use {
    super::{
        Data,
        Handler,
    },
    crate::http::{
        Request,
        Resource,
        Response,
    },
    std::{
        collections::HashMap,
        env,
        fs,
    },
};

pub struct WebService;

impl WebService {
    pub fn load_json() -> Vec<Data> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{data_path}/data/data.json");
        let json_contents = fs::read_to_string(full_path);
        let data = serde_json::from_str(
            json_contents
                .unwrap()
                .as_str(),
        )
        .unwrap();

        data
    }
}

impl Handler for WebService {
    fn handle(req: &Request) -> Response {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "data" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "applicaion/json");
                Response::new("200", Some(headers), body)
            }
            _ => Response::new(
                "404",
                None,
                Self::load_file("pages/error.html"),
            ),
        }
    }
}
