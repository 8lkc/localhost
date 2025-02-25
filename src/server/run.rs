use {
    super::Server,
    crate::{
        server::Router,
        utils::AppResult,
    },
    std::{
        io::Read,
        net::TcpListener,
    },
};

impl Server {
    pub fn has_valid_config(&self) -> bool {
        self.host.is_some()
            && self.host.is_some()
            && self.ports.is_some()
            && self.root.is_some()
            && self.error_pages.is_some()
            && self.listing.is_some()
            && self.routes.is_some()
            && self.cgi.is_some()
            && self
                .uploads_max_size
                .is_some()
        // && self
        //     .routes()
        //     .iter()
        //     .all(|route| route.has_valid_config())
    }

    pub fn run(&self) -> AppResult<()> {
        // Start a server listening on socket address
        let listener = TcpListener::bind(self.host())?;
        println!("Running on {:?}", self.host);

        // Listen to incoming connections in a loop
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue,
            };

            dbg!("Connection established!");

            let mut read_buffer = [0; 90];
            if stream
                .read(&mut read_buffer)
                .is_err()
            {
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
            Router::direct(req, &mut stream)
        }

        Ok(())
    }
}
