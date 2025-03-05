use {
    super::{Route, Router, Server},
    crate::{utils:: HTTP, Request},
};

impl Server {
    pub fn has_valid_config(&self) -> bool {
        self.host.is_some()
            && self.ports.is_some()
            && self.root.is_some()
            && self.listing.is_some()
            && self.uploads.is_some()
            && self.router.is_some()
            && self
                .router
                .as_ref()
                .unwrap()
                .has_validate_config()
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

    pub fn check_session(&self, path: &str, req: &Request) -> bool {
        if self.get_session(path) {
            if let Ok(mut http) = HTTP.write() {  
                http.has_valid_session(req)
            } else {
                false
            }
        } else {
            true
        }
    }
}

impl Route {
    pub fn has_valid_config(&self) -> bool {
        self.path.is_some()
            && self.methods.is_some()
            && self.session.is_some()
    }
}
