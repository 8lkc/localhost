use std::io::{Read, Write};

use mio::net::TcpStream;

use super::config::{EpollInstance, Server};

pub struct Localhost {addresses:Vec<String>}
impl Localhost {
    fn new(servers:&[Server]) -> Self {
        Self {
            addresses: servers.iter().map(|server| {
                server.get_ports().iter().map(|port| {format!("{}:{}", server.get_host(), port)}).collect::<Vec<String>>()
            }).flatten().collect::<Vec<String>>()
        }
    }

    fn handle_client(stream:&mut TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {}
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {return;}
            Err(err) => {
                eprintln!("Error reading from stream: {:?}", err);
                return;
            }
        }
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn start(servers:&[Server]) {
        let localhost = Localhost::new(servers);
        EpollInstance::install(&localhost.addresses, Localhost::handle_client);
    }
}
