mod handler;
mod router;

use {
    router::Router,
    serde::{Deserialize, Serialize},
    std::{io::Read, net::TcpListener},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    host: Option<String>,
    ports: Option<Vec<usize>>,
    methods: Option<Vec<String>>,
    timeout: Option<usize>,
}

impl Server {
    pub fn run(&self) -> Result<(), String> {
        // Start a server listening on socket address
        let listener = TcpListener::bind(self.host()).map_err(|e| e.to_string())?;
        println!("Running on {:?}", self.host);

        // Listen to incoming connections in a loop
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue,
            };

            dbg!("Connection established!");

            let mut read_buffer = [0; 90];
            if let Err(_) = stream.read(&mut read_buffer) {
                dbg!("Failed to read stream!");
                continue;
            }

            // Convert HTTP request to Rust data structure
            let req = match String::from_utf8(read_buffer.to_vec()) {
                Ok(req) => req,
                Err(_) => continue,
            }
            .into();

            // Route request to appropriate handler
            if let Err(_) = Router::route(req, &mut stream) {
                dbg!("Failed to direct request!");
                continue;
            }
        }

        Ok(())
    }

    pub fn has_valid_config(&self) -> bool {
        self.host.is_some()
            && self.methods.is_some()
            && self.ports.is_some()
            && self.timeout.is_some()
    }

    pub fn host(&self) -> &str {
        self.host.as_ref().unwrap()
    }

    pub fn ports(&self) -> &Vec<usize> {
        self.ports.as_ref().unwrap()
    }

    pub fn methods(&self) -> &Vec<String> {
        self.methods.as_ref().unwrap()
    }

    pub fn timeout(&self) -> usize {
        self.timeout.unwrap()
    }
}
