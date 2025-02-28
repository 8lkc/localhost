use {
    super::Handler,
    crate::{
        message::{
            Headers,
            Request,
            Resource,
            Response,
        },
        server::Middleware,
        utils::{
            AppErr,
            HttpErr,
            HttpResult,
            TEMPLATES,
        },
        Method,
    },
    tera::Context,
};

pub struct Http;

impl Handler for Http {
    fn handle(req: &Request) -> HttpResult<Response> {
        Middleware::check(req)
            .logger()?
            .method(Method::GET)?;

        // Get the path of static page resource being requested
        let Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => Self::serve_index("index.html"),
            path => Self::serve_static(path),
        }
    }
}

impl Http {
    fn serve_index(tmpl: &str) -> HttpResult<Response> {
        let mut ctx = Context::new();
        ctx.insert("title", "Rust");
        ctx.insert(
            "description",
            "A safe, concurrent, practical language",
        );

        let page = TEMPLATES
            .render(&tmpl, &ctx)
            .map_err(|e| AppErr::from(e))?;
        Ok(Response::ok(None, Some(page)))
    }

    fn serve_static(path: &str) -> HttpResult<Response> {
        let mut headers = Headers::new();
        if path.ends_with(".css") {
            headers.insert(
                "Content-Type".to_string(),
                "text/css".to_string(),
            );
        }
        else if path.ends_with(".js") {
            headers.insert(
                "Content-Type".to_string(),
                "text/javascript".to_string(),
            );
        }
        else {
            let tmpl = format!("{}.html", path);
            let ctx = Context::new();
            let page = TEMPLATES
                .render(&tmpl, &ctx)
                .map_err(|e| AppErr::from(e))?;
            return Ok(Response::ok(None, Some(page)));
        }

        let content = Self::load_file(path).ok_or(HttpErr::from(404))?;
        Ok(Response::ok(Some(headers), Some(content)))
    }
}
