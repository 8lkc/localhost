use {
    super::{
        handler::{Api, Cgi, Handler, Http, Upload},
        Route, Router,
    },
    crate::{
        message::{Request, Resource},
        utils::HttpErr,
        Method,
    },
    std::io::Write,
};

impl Route {}
impl Router {
    pub fn direct(&self, request: Request, stream: &mut impl Write) {
        // Debug print for incoming request
        println!("Full request details:");
        println!(
            "Content-Length: {}",
            request
                .headers
                .get("Content-Length")
                .unwrap_or(&"N/A".to_string())
        );
        println!("Actual body length: {}", request.body.len());
        println!(
            "Body first 100 chars: {}",
            &request.body[..100.min(request.body.len())]
        );

        let response = match &request.resource {
            Resource::Path(s) => {
                let route: Vec<&str> = s
                    .split("/")
                    .filter(|&x| !x.is_empty())
                    .collect();

                // Debug print routes
                println!("Parsed Routes: {:?}", route);

                // More flexible routing logic
                let path = route.first().unwrap_or(&"");

                if !self.check_session(path, &request) || *path == "auth" {
                    if let Some(auth_page) = self.redirect(path) {
                        return Http::serve_auth(&auth_page)
                            .unwrap_or_else(|e| e.into())
                            .send(stream);
                    }
                }

                match *path {
                    "api" => Api::handle(&request),
                    "cgi" => Cgi::handle(&request),
                    "upload" => {
                        // Explicit debug for upload
                        println!(
                            "Upload Request - Method: {:?}",
                            request.method
                        );

                        match request.method {
                            Method::GET => Upload::serve_form(),
                            Method::POST => Upload::handle(&request),
                            _ => {
                                println!("Unsupported method for upload");
                                Err(HttpErr::from(405)) // Method Not Allowed
                            }
                        }
                    }
                    _ => {
                        println!(
                            "Default route handling for path: {}",
                            path
                        );
                        Http::handle(&request)
                    }
                }
            }
        }
        .unwrap_or_else(|e| {
            // More detailed error logging
            println!("Request handling error: {:?}", e);
            e.into()
        });

        response.send(stream)
    }

    pub fn redirect(&self, path: &str) -> Option<String> {
        let reforme_path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };

        if let Some(routes) = &self.routes {
            for route in routes {
                if let Some(route_path) = &route.path {
                    let reforme_route_path = if route_path.starts_with('/')
                    {
                        route_path.to_string()
                    } else {
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
