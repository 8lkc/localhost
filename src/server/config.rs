use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    servers: Vec<Server>
} impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    pub fn get_servers(&self) -> &[Server] { &self.servers }
}

#[derive(Deserialize)]
pub struct Server {
    name: String,
    host: String,
    ports:Vec<u16>
} impl Server {
    // Just for the getters
    pub fn get_host(&self) -> &str { &self.host }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_ports(&self) -> &[u16] { &self.ports }
}
