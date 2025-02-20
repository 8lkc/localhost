use {
    super::Handler,
    crate::{
        message::Request,
        utils::{
            AppErr,
            AppResult,
            INTERPRETERS,
        },
        Resource,
        Response,
    },
    std::{
        collections::HashMap,
        path::Path,
        process::Command,
    },
};

/// Common Gateway Interface
pub struct Cgi;

impl Handler for Cgi {
    fn handle(req: &Request) -> AppResult<Response> {
        let Resource::Path(path) = &req.resource;

        let ext = path
            .split('.')
            .next_back()
            .ok_or(AppErr::ExtNotFound)?;

        let interpreter = INTERPRETERS
            .get(ext)
            .ok_or(AppErr::NoCGI)?;

        let script_path = format!("public{}", path);
        if !Path::new(&script_path).exists() {
            return Err(AppErr::NoCGI);
        }

        let query_str = path
            .split('?')
            .nth(1)
            .unwrap_or("");

        let output = Command::new(interpreter)
            .arg(&script_path)
            .env("REQUEST_METHOD", req.method.to_string())
            .env("QUERY_STRING", query_str)
            .output()?;

        if output.status.success() {
            Ok(Response::new(
                "200",
                Some(HashMap::from([(
                    "Content-Type",
                    "text/plain",
                )])),
                Some(String::from_utf8_lossy(&output.stdout).to_string()),
            ))
        }
        else {
            Err(AppErr::new(&format!(
                "CGI error: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }
}
