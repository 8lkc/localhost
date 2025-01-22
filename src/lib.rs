mod http;
mod server;

use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

pub use {http::*, server::Server};

#[derive(Debug, Serialize, Deserialize)]
pub struct Multiplexer {
    servers: Vec<Server>,
}

pub fn loader(path: &str) -> Result<Multiplexer, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let mux: Multiplexer = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(mux)
}
