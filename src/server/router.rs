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

    pub fn direct(&self, request: Request, stream: &mut impl Write) {
        let response = match &request.resource {
            Resource::Path(s) => {
                let route: Vec<&str> = s.split("/").collect();
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
}
