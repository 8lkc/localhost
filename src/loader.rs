use crate::{Multiplexer, Server};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    servers: Vec<Server>,
}

impl Config {
    pub fn servers(self) -> Vec<Server> {
        self.servers
    }

    pub fn clean(&mut self) {
        let mut idx = 1;
        self.servers.retain(|server| {
            if !server.has_valid_config() {
                println!("Invalid Config: Server number {}", idx);
            }
            idx += 1;
            server.has_valid_config()
        })
    }
}

pub struct Loader;

impl Loader {
    pub fn load(path: &'static str) -> Result<Multiplexer, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?;

        let mut config: Config = toml::from_str(&contents).map_err(|e| e.to_string())?;
        config.clean();

        let mux = Multiplexer::new(config).map_err(|e| e.to_string())?;
        Ok(mux)
    }
}
