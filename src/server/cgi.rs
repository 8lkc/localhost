use {
    super::Server,
    crate::http::Request,
    std::{
        io::Write,
        net::TcpStream,
        process::Command,
    },
};

pub struct CommonGatewayInterface;

impl CommonGatewayInterface {
    pub fn is_cgi_request(
        &self,
        request: &Request,
        servers: &Vec<Server>,
    ) -> Result<Option<String>, String> {
        let path = request.resource.path();
        let extension = path
            .split('.')
            .last()
            .ok_or_else(|| {
                "No file extension found in the request path".to_string()
            })?;

        let server = servers.iter().find(|server| {
            server
                .cgi_handler
                .as_ref()
                .map(|handlers| handlers.contains_key(extension))
                .unwrap_or(false)
        });

        match server {
            Some(server) => Ok(server
                .cgi_handler
                .as_ref()
                .and_then(|handlers| {
                    handlers
                        .get(extension)
                        .cloned()
                })),
            None => Ok(None),
        }
    }

    pub fn execute_cgi(
        &self,
        cgi_script: &str,
        request: &Request,
        stream: &mut TcpStream,
    ) -> Result<(), String> {
        if !std::path::Path::new(request.resource.path()).exists() {
            return Err(
                "CommonGatewayInterface script not found".to_string()
            );
        }
        let query_string = request
            .resource
            .path()
            .split('?')
            .nth(1)
            .unwrap_or("");
        let output = Command::new(cgi_script)
            .arg(request.resource.path())
            .env("REQUEST_METHOD", &request.method.to_string())
            .env("QUERY_STRING", query_string)
            .output()
            .map_err(|e| {
                format!(
                    "Failed to execute CommonGatewayInterface: {}",
                    e
                )
            })?;

        if output.status.success() {
            stream
                .write_all(&output.stdout)
                .map_err(|e| format!("Failed to write response: {}", e))?;
        }
        else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "CommonGatewayInterface script error: {}",
                error_message
            ));
        }

        Ok(())
    }
}
