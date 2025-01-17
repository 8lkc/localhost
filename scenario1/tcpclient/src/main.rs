use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut connection = TcpStream::connect("127.0.0.1:3000").unwrap();
    println!("Connected to server!");
    connection.write("Hello from client!".as_bytes()).unwrap();
    let mut buffer = [0; 1024];
    connection.read(&mut buffer).unwrap();
    println!("Server response: {}", str::from_utf8(&buffer).unwrap());
}
