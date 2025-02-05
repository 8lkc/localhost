#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub ports: Vec<u16>,
}

impl ServerConfig {
    pub fn new(host: String, ports: Vec<u16>) -> Self {
        ServerConfig { host, ports }
    }
}