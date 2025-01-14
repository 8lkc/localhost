use {
    std::{
        io::{
            BufRead,
            BufReader,
            Error,
            ErrorKind,
            Result,
            Write,
        },
        net::{
            TcpListener,
            TcpStream,
        },
        thread,
        time::Duration,
    },
    tera::{
        Context,
        Tera,
    },
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    Ok(listener
        .incoming()
        .for_each(|stream| handler_connection(stream.unwrap()).unwrap()))
}

fn handler_connection(mut stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&stream);
    let tera = Tera::new("./pages/*.html").map_err(|e| Error::new(ErrorKind::Other, e))?;
    let mut ctx = Context::new();

    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()?;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            ctx.insert("code", "200");
            ("HTTP/1.1 200 OK", "index.html")
        }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ctx.insert("code", "200");
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => {
            ctx.insert("code", "404");
            ctx.insert("message", "Not Found");
            ("HTTP/1.1 404 NOT FOUND", "error.html")
        }
    };

    let contents = tera
        .render(&filename, &ctx)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())
}
