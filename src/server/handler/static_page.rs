use {
    super::Handler,
    crate::http::{
        Request,
        Resource,
        Response,
    },
    std::collections::HashMap,
};

pub struct StaticPage;

impl Handler for StaticPage {
    fn handle(req: &Request) -> Response {
        // Get the path of static page resource being requested
        let Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => Response::new(
                "200",
                None,
                Self::load_file("pages/index.html"),
            ),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map = HashMap::new();

                    if contents.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    }
                    else if contents.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    }
                    else {
                        map.insert("Content-Type", "text/html");
                    }

                    Response::new("200", Some(map), Some(contents))
                }
                None => Response::new(
                    "404",
                    None,
                    Self::load_file("pages/error.html"),
                ),
            },
        }
    }
}
