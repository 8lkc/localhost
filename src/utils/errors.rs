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
pub enum AppErr {
    DeserializeTOML(toml::de::Error),
    SerDeJSON(serde_json::Error),
    NonBlocking(io::Error),
    ParseAddress(AddrParseError),
    Other(io::Error),
    Custom(String),
}

/// Custom `Result` specific to this crate.
pub type AppResult<T> = result::Result<T, AppErr>;

impl Display for AppErr {
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

impl Error for AppErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NonBlocking(e) => Some(e),
            Self::SerDeJSON(e) => Some(e),
            Self::DeserializeTOML(e) => Some(e),
            Self::ParseAddress(e) => Some(e),
            Self::Other(e) => Some(e),
            Self::Custom(_) => None,
        }
    }
}

impl From<io::Error> for AppErr {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            ErrorKind::WouldBlock => Self::NonBlocking(value),
            _ => Self::Other(value),
        }
    }
}

impl From<toml::de::Error> for AppErr {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializeTOML(value)
    }
}

impl From<AddrParseError> for AppErr {
    fn from(value: AddrParseError) -> Self { Self::ParseAddress(value) }
}

impl From<serde_json::Error> for AppErr {
    fn from(value: serde_json::Error) -> Self { Self::SerDeJSON(value) }
}

impl AppErr {
    pub fn new(msg: &str) -> Self { Self::Custom(msg.to_string()) }

    pub fn last_os_error() -> Self {
        Self::Other(io::Error::last_os_error())
    }
}
