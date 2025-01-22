use localhost::{loader, Multiplexer};

fn main() {
    let mux = match loader("./config/server.toml") {
        Ok(m) => m,
        Err(e) => {
            println!("{e}");
        }
    };
}
