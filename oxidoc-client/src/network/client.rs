use crate::errors::ClientError;
use std::net::TcpStream;

pub fn connect_to_server(server: String) -> Result<TcpStream, ClientError> {
    TcpStream::connect(server).map_err(|e| ClientError::Connection(e.to_string()))
}
