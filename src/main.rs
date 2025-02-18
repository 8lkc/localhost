use server::{Config, Localhost};

mod server;

fn main() {
    let config = Config::from_file("src/server/config.toml");
    Localhost::start(config.get_servers());
}
