use {
    super::Handler,
    crate::{
        message::Request,
        server::Middleware,
        utils::{
            AppErr,
            HttpErr,
            HttpResult,
            INTERPRETERS,
        },
        Method,
        Resource,
        Response,
    },
    std::{
        path::Path,
        process::Command,
    },
};

/// Common Gateway Interface
pub struct Cgi;

impl Handler for Cgi {
    fn handle(req: &Request) -> HttpResult<Response> {
        Middleware::check(req)
            .logger()?
            .method(Method::GET)?;

        let Resource::Path(path) = &req.resource;

        let ext = path
            .split('.')
            .next_back()
            .ok_or(AppErr::ExtNotFound)?;

        let interpreter = INTERPRETERS
            .get(ext)
            .ok_or(AppErr::NoCGI)?;

        let script = format!(
            "{}/public{}",
            env!("CARGO_MANIFEST_DIR"),
            path
        );
        if !Path::new(&script).exists() {
            return Err(HttpErr::from(AppErr::NoCGI));
        }

        let query_str = path
            .split('?')
            .nth(1)
            .unwrap_or("");

        let output = Command::new(interpreter)
            .arg(&script)
            .env("REQUEST_METHOD", req.method.to_string())
            .env("QUERY_STRING", query_str)
            .output()?;

        let body = String::from_utf8_lossy(&output.stdout).to_string();

        if output.status.success() {
            Ok(Response::ok(None, Some(body)))
        }
        else {
            Err(HttpErr::from(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}
