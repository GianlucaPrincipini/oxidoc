use crate::collection::Document;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Response {
    Success(String),
    Doc(Option<Document>),
    Failure(String),
    Ack,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Success(msg) => write!(f, "Success: {}", msg),
            Response::Doc(Some(doc)) => write!(f, "Document: {}", serde_json::to_string(doc).unwrap_or_else(|_| "<invalid>".to_string())),
            Response::Doc(None) => write!(f, "Document: None"),
            Response::Failure(msg) => write!(f, "Server responded with failure: {}", msg),
            Response::Ack => write!(f, "Acknowledged"),
        }
    }
}

impl Response {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Response::Success(msg) => msg.as_bytes().to_vec(),
            Response::Doc(Some(doc)) => serde_json::to_vec(doc).expect("Serialization failed."),
            Response::Doc(None) => b"2".to_vec(),
            Response::Failure(msg) => {
                let mut r = msg.as_bytes().to_vec();
                r.insert(0, b'!');
                r
            }
            Response::Ack => b"ACK".to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes == b"ACK" {
            Response::Ack
        } else if bytes == b"2" {
            Response::Doc(None)
        } else if bytes.starts_with(b"{") && bytes.ends_with(b"}") {
            match serde_json::from_slice::<Document>(bytes) {
                Ok(doc) => Response::Doc(Some(doc)),
                Err(e) => Response::Failure(format!("Deserialization failed for Document: {}", e)),
            }
        } else if bytes.starts_with(b"!") {
            Response::Failure(String::from_utf8_lossy(&bytes[1..]).to_string())
        } else {
            Response::Success(String::from_utf8_lossy(bytes).to_string())
        }
    }
}

