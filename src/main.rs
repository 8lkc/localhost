use std::{
    fs,
    io::{
        self,
        BufRead,
        BufReader,
        Write,
    },
    net::{
        TcpListener,
        TcpStream,
    },
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    Ok(listener
        .incoming()
        .for_each(|stream| handler_connection(stream.unwrap()).unwrap()))
}

fn handler_connection(mut stream: TcpStream) -> io::Result<()> {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()?;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "pages/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "pages/error.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())
}
