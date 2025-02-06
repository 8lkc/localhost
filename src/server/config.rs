use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Servers {
    pub servers: Vec<ServerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    pub ports: Vec<u16>,
}

impl Servers {
    pub fn new() -> Self {
        Self::read_config()
    }

    fn read_config() -> Self {
        let config = std::fs::read_to_string("config.yaml").unwrap();
        serde_yaml::from_str::<Servers>(&config).expect("ERROR")
    }
}

// impl ServerConfig {
//     pub fn new() -> Self {
//         Self::read_config()
//     }

//     fn read_config() -> Self {
//         // let config = std::fs::read_to_string("config.toml").unwrap();
//         let config = std::fs::read_to_string("config.yaml").unwrap();
//         // let checker = serde_yaml::from_str::<ServerConfig>(&config).expect("ERROR");
//         serde_yaml::from_str::<ServerConfig>(&config).expect("ERROR")
//         // println!("{:?}", checker);
//         // checker
//     }
// }