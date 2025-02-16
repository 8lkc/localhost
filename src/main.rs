use std::io;

use server::{Config, Localhost};

mod server;

fn main() -> io::Result<()> {
    let config = Config::from_file("src/server/config.toml")?;
    let localhost = Localhost::new(config.get_servers())?;
    localhost.start()
}
