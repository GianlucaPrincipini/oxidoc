use crate::network::handler::handle_stream;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn start_listener(address: &str, port: u16) -> Result<(), Error> {
    let full_address: String = format!("{address}:{port}");
    let listener = TcpListener::bind(full_address)?;
    println!("Server listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream)
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    fn get_free_port() -> u16 {
        // Setting port 0 lets the OS assign a free port
        TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    #[test]
    fn listener_accepts_connection_and_sends_welcome() {
        let port = get_free_port();
        let handle = thread::spawn(move || {
            start_listener("127.0.0.1", port).unwrap();
        });
        // Wait a moment for the oxidoc-server to start
        thread::sleep(Duration::from_millis(100));
        let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("Thread connection failed");
        let mut buf = [0u8; 64];
        let n = stream.read(&mut buf).unwrap();
        let msg = std::str::from_utf8(&buf[..n]).unwrap();
        assert!(msg.contains("Welcome to oxidoc!"));
        drop(stream);
    }
}
