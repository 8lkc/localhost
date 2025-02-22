use {
    super::Handler,
    crate::{
        message::{
            Request,
            Resource,
            Response,
        },
        utils::{
            HttpErr,
            HttpResult,
        },
    },
    std::collections::HashMap,
};

pub struct Http;

impl Handler for Http {
    fn handle(req: &Request) -> HttpResult<Response> {
        // Get the path of static page resource being requested
        let Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => Ok(Response::new(
                200,
                None,
                Self::load_file("index.html"),
            )),
            path => match Self::load_file(path) {
                Some(mut contents) => {
                    let mut map = HashMap::new();

                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    }
                    else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    }
                    else {
                        contents = format!("{}.html", contents);
                        map.insert("Content-Type", "text/html");
                    }

                    Ok(Response::new(200, Some(map), Some(contents)))
                }
                None => Ok(Response::from(HttpErr::from(404))),
            },
        }
    }
}
