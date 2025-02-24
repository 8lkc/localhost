

// use std::io::{Read, Write};

// use mio::net::TcpStream;

// use crate::server::Server;

// // use super::{config::{EpollInstance, Server}, errors::ServerError};

// pub struct Localhost {
//     addresses: Vec<String>
// } impl Localhost {
//     fn new(servers:&[Server]) -> Self {
//         Self {
//             addresses: servers.iter().map(|server| {
//                 server.get_ports().iter().map(|port| {format!("{}:{}", server.get_host(), port)}).collect::<Vec<String>>()
//             }).flatten().collect::<Vec<String>>()
//         }
//     }

//     fn handle_client(stream: &mut TcpStream) {
//         let mut buffer = [0; 1024];
//         match stream.read(&mut buffer) {
//             Ok(_) => {
//                 // Basic response for now - we'll improve this later
//                 let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
//                 if let Err(err) = stream.write_all(response.as_bytes()) {
//                     eprintln!("Error writing to stream: {:?}", err);
//                     return;
//                 }
//                 if let Err(err) = stream.flush() { eprintln!("Error flushing stream: {:?}", err); }
//             }
//             Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => return,
//             Err(err) => {
//                 eprintln!("Error reading from stream: {:?}", err);
//                 return;
//             }
//         }
//     }

//     pub fn start(servers:&[Server]) -> Result<(), ServerError> {
//         let localhost = Localhost::new(servers);
//         EpollInstance::install(&localhost.addresses, Localhost::handle_client)
//     }
// }
