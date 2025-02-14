use std::{fs, io, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Config {
    servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Server {
    name: String,
    host: String,
    ports: Vec<u16>,
}

impl Config {
    pub(super) fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        Ok(config)
    }
}

// Just the getters
impl Server {
    pub(super) fn get_host(&self) -> &str {&self.host}
    pub(super) fn get_name(&self) -> &str {&self.name}
    pub(super) fn get_ports(&self) -> &[u16] {&self.ports}
}
