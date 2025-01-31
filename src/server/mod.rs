pub mod cgi;
mod handler;
pub mod router;
// mod test;

use {
    router::Router,
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        collections::HashMap,
        io::Read,
        net::{
            SocketAddr,
            TcpListener,
        },
        str::FromStr,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    host:             Option<String>,
    ports:            Option<Vec<usize>>,
    root:             Option<String>,
    error_pages:      Option<Vec<String>>,
    uploads_max_size: Option<u64>,
    cgi_handler:      Option<HashMap<String, String>>,
    listing:          Option<bool>,
    router:           Option<Router>,
}

impl Server {
    pub fn has_valid_config(&self) -> bool {
        self.host.is_some()
            && self.host.is_some()
            && self.ports.is_some()
            && self.root.is_some()
            && self.error_pages.is_some()
            && self
                .uploads_max_size
                .is_some()
            && self.cgi_handler.is_some()
            && self.listing.is_some()
            && self.router.is_some()
    }

    pub fn run(&self) -> Result<(), String> {
        // Start a server listening on socket address
        let listener =
            TcpListener::bind(self.host()).map_err(|e| e.to_string())?;
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
            if let Err(_) = self
                .router()
                .run(req, &mut stream)
            {
                dbg!("Failed to direct request!");
                continue;
            }
        }

        Ok(())
    }

    pub fn host(&self) -> &str {
        dbg!(self.host.as_ref());
        self.host.as_ref().unwrap()
    }

    pub fn ports(&self) -> &Vec<usize> { self.ports.as_ref().unwrap() }

    pub fn root(&self) -> &str { self.root.as_ref().unwrap() }

    pub fn error_pages(&self) -> &Vec<String> {
        self.error_pages
            .as_ref()
            .unwrap()
    }

    pub fn uploads_max_size(&self) -> u64 {
        self.uploads_max_size.unwrap()
    }

    pub fn cgi_handler(&self) -> &HashMap<String, String> {
        self.cgi_handler
            .as_ref()
            .unwrap()
    }

    pub fn listing(&self) -> bool { self.listing.unwrap() }

    pub fn router(&self) -> &Router { self.router.as_ref().unwrap() }

    pub fn listeners(&self) -> Result<Vec<TcpListener>, String> {
        let mut listeners = vec![];
        let host = self.host();

        for port in self.ports() {
            let address =
                SocketAddr::from_str(format!("{host}:{port}").as_str())
                    .map_err(|e| e.to_string())?;

            dbg!(&address);

            match TcpListener::bind(address) {
                Ok(listener) => {
                    println!("Listener created on {}:{}", host, port);
                    listeners.push(listener);
                }
                Err(e) => {
                    dbg!(e.to_string());
                    ()
                }
            };
        }

        Ok(listeners)
    }
}
