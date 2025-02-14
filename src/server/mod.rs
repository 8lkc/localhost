pub mod cgi;
mod handler;
pub mod router;

use {
    crate::utils::AppResult,
    router::{
        Route,
        Router,
    },
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

#[derive(Serialize, Deserialize)]
pub struct Server {
    host:             Option<String>,
    ports:            Option<Vec<usize>>,
    root:             Option<String>,
    error_pages:      Option<Vec<String>>,
    uploads_max_size: Option<u64>,
    cgi_handler:      Option<HashMap<String, String>>,
    listing:          Option<bool>,
    routes:           Option<Vec<Route>>,
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
            && self.routes.is_some()
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
            if Router::run(req, &mut stream).is_err() {
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

    pub fn routes(&self) -> &Vec<Route> { self.routes.as_ref().unwrap() }

    pub fn listeners(&self) -> AppResult<Vec<TcpListener>> {
        let mut listeners = vec![];
        let host = self.host();

        for port in self.ports() {
            let address =
                SocketAddr::from_str(format!("{host}:{port}").as_str())?;

            dbg!(&address);

            match TcpListener::bind(address) {
                Ok(listener) => {
                    println!("Listener created on {}:{}", host, port);
                    listeners.push(listener);
                }
                Err(e) => {
                    dbg!(e.to_string());
                }
            };
        }

        Ok(listeners)
    }
}
