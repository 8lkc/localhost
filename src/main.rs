use std::{
    fs,
    io::{
        BufRead,
        BufReader,
        Write,
    },
    net::{
        TcpListener,
        TcpStream,
    },
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handler_connection(stream)
    }
}

fn handler_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("index.html").unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream
        .write_all(response.as_bytes())
        .unwrap()
}
