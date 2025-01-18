mod handler;
mod router;

use {
    router::Router,
    std::{
        io::{
            Read,
            Result,
        },
        net::TcpListener,
    },
};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server {
            socket_addr,
        }
    }

    pub fn run(&self) -> Result<()> {
        // Start a server listening on socket address
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        // Listen to incoming connections in a loop
        for stream in listener.incoming() {
            let mut stream = stream?;
            println!("Connection established");

            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer)?;

            // Convert HTTP request to Rust data structure
            let req = String::from_utf8(read_buffer.to_vec())
                .unwrap()
                .into();

            // Route request to appropriate handler
            Router::route(req, &mut stream)?
        }

        Ok(())
    }
}
