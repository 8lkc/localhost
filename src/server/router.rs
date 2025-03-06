use {
    super::{
        handler::{
            Api,
            Cgi,
            Handler,
            Http,
            Upload,
        },
        Route,
        Router,
    },
    crate::{
        message::{
            Method,
            Request,
            Resource,
            Response,
        },
        utils::{
            HttpErr,
            HTTP,
        },
    },
};

impl Route {
    pub fn has_valid_config(&self) -> bool {
        self.path.is_some() && self.methods.is_some() && self.session.is_some()
    }

    pub fn path(&self) -> &str { self.path.as_ref().unwrap() }

    pub fn allowed_methods(&self) -> &Vec<String> { self.methods.as_ref().unwrap() }

    pub fn check_session(&self) -> bool { self.session.unwrap() }
}

impl Router {
    pub fn has_validate_config(&self) -> bool {
        if let Some(cgi) = &self.cgi {
            if !cgi.has_valid_config() {
                return false;
            }
        }
        self.routes.is_some()
            && self
                .routes
                .as_ref()
                .unwrap()
                .iter()
                .all(|route| route.has_valid_config())
    }

    pub fn get_session(&self, path: &str) -> bool {
        for route in self.routes() {
            if let Some(path_route) = &route.path {
                if path_route == path {
                    return route.check_session();
                }
            }
        }
        false
    }

    fn check_session(&self, path: &str, req: &Request) -> bool {
        if self.get_session(path) {
            if let Ok(mut http) = HTTP.write() {
                http.has_valid_session(req)
            }
            else {
                false
            }
        }
        else {
            true
        }
    }

    pub(crate) fn direct(&self, request: &Request) -> Response {
        match &request.resource {
            Resource::Path(s) => {
                let route: Vec<&str> = s
                    .split("/")
                    .filter(|&x| !x.is_empty())
                    .collect();

                // More flexible routing logic
                let path = route.first().unwrap_or(&"");
                println!("<=================================>");
                println!("<=====Path : {}=====>",path);
                println!("<=================================>");
                
                if !self.check_session(path, &request) || *path == "auth" {
                    if let Some(auth_page) = self.redirect(path) {
                        return Http::serve_auth(&auth_page).unwrap_or_else(|e| e.into());
                    };
                };

                match *path {
                    "api" => Api::handle(&request),
                    "cgi" => Cgi::handle(&request),
                    "upload" => match request.method {
                        Method::GET => Upload::serve_form(),
                        Method::POST => Upload::handle(&request),
                        _ => Err(HttpErr::from(405)),
                    },
                    _ => Http::handle(&request),
                }
            }
        }
        .unwrap_or_else(|e| e.into())
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
                    let reforme_route_path = if route_path.starts_with('/') {
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

    fn routes(&self) -> &Vec<Route> { self.routes.as_ref().unwrap() }
}
