use {
    super::Handler,
    crate::http::{Request, Resource, Response},
    std::collections::HashMap,
};

pub struct StaticPage;

impl Handler for StaticPage {
    fn handle(req: &Request) -> Result<Response, String> {
        // Get the path of static page resource being requested
        let Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => Ok(Response::new("200", None, Self::load_file("index.html"))),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map = HashMap::new();

                    if contents.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if contents.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }

                    Ok(Response::new("200", Some(map), Some(contents)))
                }
                None => Ok(Response::new("404", None, Self::load_file("error.html"))),
            },
        }
    }
}
