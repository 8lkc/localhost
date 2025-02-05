use {
    super::Server,
    crate::http::Request,
    std::{io::Write, net::TcpStream, process::Command},
};

pub struct CommonGatewayInterface;

impl CommonGatewayInterface {
    pub fn is_cgi_request(
        &self,
        request: &Request,
        servers: &[Server],
    ) -> Result<Option<String>, String> {
        let path = request.resource.path();
        let extension = path
            .split('.')
            .next_back()
            .ok_or_else(|| "No file extension found in the request path".to_string())?;

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
                .and_then(|handlers| handlers.get(extension).cloned())),
            None => Ok(None),
        }
    }

    pub fn execute_cgi(
        &self,
        cgi_script: &str,
        request: &Request,
        stream: &mut TcpStream,
    ) -> Result<(), String> {
        let script_path = format!("public{}", request.resource.path());
        if !std::path::Path::new(&script_path).exists() {
            dbg!(request.resource.path());
            return Err("CommonGatewayInterface script not found".to_string());
        }
        let query_string = request.resource.path().split('?').nth(1).unwrap_or("");
        dbg!(cgi_script);
        let interpreter = self.get_interpreter(&script_path)?;
        let output = Command::new(interpreter)
            .arg(&format!("public{}", request.resource.path()))
            .env("REQUEST_METHOD", request.method.to_string())
            .env("QUERY_STRING", query_string)
            .output()
            .map_err(|e| format!("Failed to execute CommonGatewayInterface: {}", e))?;

        if output.status.success() {
            stream
                .write_all(&output.stdout)
                .map_err(|e| format!("Failed to write response: {}", e))?;
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "CommonGatewayInterface script error: {}",
                error_message
            ));
        }

        Ok(())
    }

    fn get_interpreter(&self, script_path: &str) -> Result<&str, String> {
        let extension = script_path
            .split('.')
            .last()
            .ok_or_else(|| "No file extension found".to_string())?;

        match extension {
            "py" => Ok("python3"),
            "php" => Ok("php"),
            "pl" => Ok("perl"),
            _ => Err(format!("Unsupported CGI script extension: {}", extension)),
        }
    }
}
