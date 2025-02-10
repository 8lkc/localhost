use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub servers: Vec<ServerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub ports: Vec<u16>,
}

impl Config {
    pub fn new() -> Self {Self::read_config()}

    fn read_config() -> Self {
        let config = std::fs::read_to_string("config.yaml").unwrap();
        serde_yaml::from_str::<Config>(&config).expect("ERROR")
    }
}
