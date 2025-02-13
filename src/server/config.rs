use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct Config {
    pub servers: Vec<ServerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ServerConfig {
    pub name: String,
    pub host: String,
    pub ports: Vec<u16>,
}

impl Config {
    pub(super) fn new() -> Self {Self::read_config()}

    fn read_config() -> Self {
        let config = std::fs::read_to_string("src/server/config.yml").unwrap();
        serde_yaml::from_str::<Config>(&config).expect("ERROR")
    }
}
