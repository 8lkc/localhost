use std::error::Error;

use server::{Config, Localhost};

fn main() -> Result<(), Box<dyn Error>> {
    // Load configuration
    let config = Config::from_file("src/server/config.toml")
        .map_err(|e| format!("Configuration error: {:?}", e))?;

    // Create and start server
    let mut localhost = Localhost::new(config.get_servers())?;
    localhost.start()?;

    Ok(())
}
