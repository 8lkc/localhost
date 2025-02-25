use std::{collections::HashMap, error::Error, fs};

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
    ports:Vec<u16>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    allowed_methods: Option<Vec<String>>,
    #[serde(default)]
    default_version: Option<String>,
    #[serde(default)]
    default_headers: Option<HashMap<String, String>>
    // #[serde(default)] :=> to make a field optional
} impl Server {
    // Just for the getters
    pub fn get_allowed_methods(&self) -> &[String] {
        // Return an empty slice if no allowed methods are set.
        self.allowed_methods.as_deref().unwrap_or(&[])
    }
    pub fn get_default_version(&self) -> &str { self.default_version.as_deref().unwrap_or("HTTP/1.1") }
    pub fn get_default_headers(&self) -> Option<&HashMap<String, String>> { self.default_headers.as_ref() }
    pub fn get_host(&self) -> &str { &self.host }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_path(&self) -> &str {
        // Default to "/" if not specified.
        self.path.as_deref().unwrap_or("/")
    }
    pub fn get_ports(&self) -> &[u16] { &self.ports }
}
