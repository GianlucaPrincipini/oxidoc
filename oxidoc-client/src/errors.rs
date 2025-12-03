use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    ArgParse(String),
    Connection(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::ArgParse(e) => write!(f, "Argument parsing error: {}", e),
            ClientError::Connection(e) => write!(f, "Connection error: {}", e),
        }
    }
}
