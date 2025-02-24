use std::io;

#[derive(Debug)]
pub enum ServerError {
    Io(std::io::Error),
    Configuration(String)
} impl From<io::Error> for ServerError {
    fn from(value:io::Error) -> Self { ServerError::Io(value) }
}
