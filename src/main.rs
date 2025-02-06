mod server;

use server::Server;
use server::config::*;

fn main() -> std::io::Result<()> {
    // Configuration de base
    let configs = Servers::new();

    // Ensure there is at least one configuration and select it.
    let config = configs.servers.first().expect("No server configuration found").clone();

    // Create and start the server using the single ServerConfig.
    let mut server = Server::new(config)?;
    println!("Server starting on 127.0.0.1:8080");
    server.start()
}