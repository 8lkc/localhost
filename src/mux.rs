use crate::Server;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Multiplexer {
    pub servers: Vec<Server>,
}

impl Default for Multiplexer {
    fn default() -> Self {
        Self { servers: vec![] }
    }
}

impl Multiplexer {
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
