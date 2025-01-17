use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer =[0; 5];
        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
        
    }
}