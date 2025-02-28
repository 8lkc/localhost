use {
    super::{
        handler::{Api, Cgi, Handler, Http},
        Route, Router, SESSION_STORE,
    },
    crate::message::{Request, Resource},
    std::io::Write,
};

impl Route {
    pub fn has_valid_config(&self) -> bool {
        self.path.is_some()
            && self.methods.is_some()
            && self.check_session.is_some()
    }
}

impl Router {
    pub fn has_validate_config(&self) -> bool {
        self.routes.is_some()
            && self
                .routes
                .as_ref()
                .unwrap()
                .iter()
                .all(|route| route.has_valid_config())
    }

    fn is_valid_session(req: &Request) -> bool {
        if let Some(cookie) = req.headers.get("Cookie") {
            if let Some(session_id) = Self::get_session_id(cookie) {
                return SESSION_STORE.validate_session(&session_id);
            }
        }
        false
    }
    fn get_session_id(cookie: &str) -> Option<String> {
        cookie
            .split(';')
            .find(|s| {
                s.trim()
                    .starts_with("session_id=")
            })
            .map(|s| s.trim()["session_id=".len()..].to_string())
    }

    pub fn direct(&self, request: Request, stream: &mut impl Write) {
        let response = match &request.resource {
            Resource::Path(s) => {
                let route: Vec<&str> = s.split("/").collect();
                let path =
                    if route[1].is_empty() { "/" } else { route[1] };
                if !self.check_session(path, &request) || path == "auth" {
                    dbg!("trouver");
                    if let Some(auth_page) = self.redirect(path) {
                        return Http::serve_auth(&auth_page)
                            .unwrap_or_else(|e| e.into())
                            .send(stream);
                    } else {
                        dbg!("non trouver", self.redirect(path));
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

    pub fn get_session(&self, path: &str) -> bool {
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(path_route) = &route.path {
                    if path_route == path {
                        return route
                            .check_session
                            .unwrap_or(false);
                    }
                }
            }
        }
        false
    }

    pub fn check_session(&self, path: &str, req: &Request) -> bool {
        if self.get_session(path) {
            Self::is_valid_session(req)
        } else {
            true
        }
    }

    pub fn redirect(&self, path: &str) -> Option<String> {
        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    if route_path == path {
                        if let Some(redirects) = &route.redirect {
                            return redirects
                                .get("/auth")
                                .cloned();
                        }
                        
                        if let Some(_default_file) = &route.default_file {
                            return route.default_file.clone();
                        }
                    }
                }
            }
        }
        None
    }
}
