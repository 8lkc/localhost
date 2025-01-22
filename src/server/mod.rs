mod handler;
mod router;

use {
    router::Router,
    std::{
        io::Read,
        net::TcpListener,
    },
};

pub struct Server<'a> {
    host: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(host: &'a str) -> Self {
        Server {
            host,
        }
    }

    pub fn run(&self) {
        // Start a server listening on socket address
        let listener = TcpListener::bind(self.host).unwrap();
        println!("Running on {}", self.host);

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
