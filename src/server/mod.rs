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
    pub fn run(&self) {
        // Start a server listening on socket address
        let listener: TcpListener = TcpListener::bind(self.host.as_ref().unwrap()).unwrap();
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
    }
}
