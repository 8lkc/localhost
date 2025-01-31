use {
    super::handler::{
        ErrorPage,
        Handler,
        StaticPage,
        WebService,
    },
    crate::http::{
        Method,
        Request,
        Resource,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        collections::HashMap,
        io::prelude::*,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    path:          Option<String>,
    method:        Option<Vec<String>>,
    default_file:  Option<String>,
    check_session: Option<bool>,
    redirect:      Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Router {
    routes: Option<Vec<Route>>,
}

impl Router {
    pub fn has_valid_config(&self) -> bool {
        match &self.routes {
            Some(routes) => routes.iter().all(|route| {
                route.path.is_some()
                    && route.method.is_some()
                    && route.default_file.is_some()
                    && route.check_session.is_some()
                    && route.redirect.is_some()
            }),
            None => false,
        }
    }

    pub fn run(
        &self,
        req: Request,
        stream: &mut impl Write,
    ) -> Result<(), String> {
        match (&req.method, &req.resource) {
            (Method::GET, Resource::Path(s)) => {
                let route: Vec<&str> = s.split("/").collect();

                match route[1] {
                    "api" => WebService::handle(&req)?
                        .send_response(stream)
                        .map_err(|e| e.to_string()),
                    _ => StaticPage::handle(&req)?
                        .send_response(stream)
                        .map_err(|e| e.to_string()),
                }
            }
            _ => ErrorPage::handle(&req)?
                .send_response(stream)
                .map_err(|e| e.to_string()),
        }
    }
}
