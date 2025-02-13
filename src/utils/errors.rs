use std::{
     error::{
          self,
          Error,
     },
     fmt::Display,
     io::{
          self,
          ErrorKind,
     },
     net::AddrParseError,
     result,
};

#[derive(Debug)]
pub enum Err {
     DeserializeTOML(toml::de::Error),
     SerDeJSON(serde_json::Error),
     NonBlocking(io::Error),
     ParseAddress(AddrParseError),
     Custom(&'static str),
     Other(io::Error),
}

/// Custom `Result` specific to this crate.
pub type Result<T> = result::Result<T, Err>;

impl Display for Err {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          match self {
               Self::Custom(msg) => writeln!(f, "Error: {msg}."),
               Self::DeserializeTOML(e) => {
                    writeln!(f, "TOML Deserialisation: {e}.")
               }
               Self::SerDeJSON(e) => {
                    writeln!(f, "JSON Serialisation/Deserialisation: {e}.")
               }
               Self::NonBlocking(e) => {
                    writeln!(f, "Non Blocking Mode: {e}.")
               }
               Self::ParseAddress(e) => {
                    writeln!(f, "Address Parsing: {e}.")
               }
               Self::Other(e) => writeln!(f, "ERROR: {e}."),
          }
     }
}

impl Error for Err {
     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
          match self {
               Self::NonBlocking(e) => Some(e),
               _ => None,
          }
     }
}

impl From<io::Error> for Err {
     fn from(value: io::Error) -> Self {
          match value.kind() {
               ErrorKind::WouldBlock => Self::NonBlocking(value),
               _ => Self::Other(value),
          }
     }
}

impl From<toml::de::Error> for Err {
     fn from(value: toml::de::Error) -> Self {
          Self::DeserializeTOML(value)
     }
}

impl From<AddrParseError> for Err {
     fn from(value: AddrParseError) -> Self { Self::ParseAddress(value) }
}

impl From<serde_json::Error> for Err {
     fn from(value: serde_json::Error) -> Self { Self::SerDeJSON(value) }
}
