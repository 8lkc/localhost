mod server;

use server::Server;
use server::config::ServerConfig;

fn main() -> std::io::Result<()> {
    // Configuration de base
    let config = ServerConfig::new(String::from("127.0.0.1"), vec![8080]);
    
    // Création et démarrage du serveur
    let mut server = Server::new(config)?;
    println!("Server starting on 127.0.0.1:8080");
    server.start()
}