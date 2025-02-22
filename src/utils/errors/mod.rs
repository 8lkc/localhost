mod app;
mod http;

use std::{
    io,
    net::AddrParseError,
    result,
};

#[derive(Debug)]
pub enum AppErr {
    DeserializeTOML(toml::de::Error),
    SerDeJSON(serde_json::Error),
    NonBlocking(io::Error),
    ParseAddr(AddrParseError),
    Other(io::Error),
    Custom(String),
    ExtNotFound,
    NoCGI,
}

pub struct HttpErr {
    status_code: u16,
    message:     String,
}

/// Custom `Result` specific to this crate.
pub type AppResult<T> = result::Result<T, AppErr>;
