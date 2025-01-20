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
    std::io::prelude::*,
};

pub struct Router;

impl Router {
    pub fn route(req: Request, stream: &mut impl Write) -> Result<(), String> {
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
