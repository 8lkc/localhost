pub mod config;

use std::io;
use std::net::TcpListener;
use self::config::ServerConfig;

pub struct Server {
    config: ServerConfig,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new(config: ServerConfig) -> io::Result<Self> {
        Ok(Server {
            config,
            listener: None,
        })
    }

    pub fn start(&mut self) -> io::Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.ports[0]);
        let listener = TcpListener::bind(&addr)?;
        
        println!("Listening on {}", addr);

        self.listener = Some(listener);

        // Boucle principale simple
        loop {
            if let Some(listener) = &self.listener {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        println!("New connection from: {}", addr);
                        self.handle_connection(stream)?;
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: std::net::TcpStream) -> io::Result<()> {
        use std::io::{Read, Write};
        
        // Buffer pour lire la requête
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        // Réponse HTTP basique
        let response = "HTTP/1.1 200 OK\r\n\
                       Content-Length: 13\r\n\
                       \r\n\
                       Hello, World!";

        stream.write(response.as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}