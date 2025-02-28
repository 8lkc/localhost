use {
    super::{
        handler::{
            Api,
            Cgi,
            Handler,
            Http,
        },
        Route,
        Router,
    },
    crate::message::{
        Request,
        Resource,
    },
    std::io::Write,
};

impl Route {}

impl Router {
    pub fn direct(&self, request: Request, stream: &mut impl Write) {
        let response = match &request.resource {
            Resource::Path(s) => {
                let route: Vec<&str> = s.split("/").collect();
                let path =
                    if route[1].is_empty() { "/" } else { route[1] };
                if !self.check_session(path, &request) || path == "auth" {
                    if let Some(auth_page) = self.redirect(path) {
                        return Http::serve_auth(&auth_page)
                            .unwrap_or_else(|e| e.into())
                            .send(stream);
                    }
                }
                match route[1] {
                    "api" => Api::handle(&request),
                    "cgi" => Cgi::handle(&request),
                    _ => Http::handle(&request),
                }
            }
        }
        .unwrap_or_else(|e| {
            // debug!(e);
            e.into()
        });

        response.send(stream)
    }

    pub fn redirect(&self, path: &str) -> Option<String> {
        let reforme_path = if path.starts_with('/') {
            path.to_string()
        }
        else {
            format!("/{}", path)
        };
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    let reforme_route_path = if route_path.starts_with('/')
                    {
                        route_path.to_string()
                    }
                    else {
                        format!("/{}", route_path)
                    };

                    if reforme_route_path == reforme_path {
                        if let Some(redirects) = &route.redirect {
                            return redirects
                                .get("/auth")
                                .cloned();
                        }
                        if let Some(default_file) = &route.default_file {
                            return Some(default_file.clone());
                        }
                    }
                }
            }
        }
        None
    }
}
