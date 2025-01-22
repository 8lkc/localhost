use std::{fs::File, io::Read};
use crate::Multiplexer;

pub struct Loader;

impl Loader {
    pub fn load(path: &'static str) -> Result<Multiplexer, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?;

        let mut mux: Multiplexer = toml::from_str(&contents).map_err(|e| e.to_string())?;

        mux.check();

        Ok(mux)
    }
}
