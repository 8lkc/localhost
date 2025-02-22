use {
    super::handler::{
        Api,
        Cgi,
        ErrorPage,
        Handler,
        Http,
    },
    crate::{
        message::{
            Method,
            Request,
            Resource,
        },
        utils::AppResult,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        collections::HashMap,
        io::Write,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Route {
    path:          Option<String>,
    method:        Option<Vec<String>>,
    default_file:  Option<String>,
    check_session: Option<bool>,
    redirect:      Option<HashMap<String, String>>,
}

impl Route {
    pub fn has_valid_config(&self) -> bool {
        self.path.is_some()
            && self.method.is_some()
            && self.default_file.is_some()
            && self.check_session.is_some()
    }
}

pub struct Router;

impl Router {
    pub fn direct(
        request: Request,
        stream: &mut impl Write,
    ) -> AppResult<()> {
        let response = match (&request.method, &request.resource) {
            (Method::GET, Resource::Path(s)) => {
                let route: Vec<&str> = s.split("/").collect();
                match route[1] {
                    "api" => Api::handle(&request),
                    "cgi" => Cgi::handle(&request),
                    _ => Http::handle(&request),
                }
            }
            _ => ErrorPage::handle(&request),
        }?;

        response.send(stream)
    }
}
