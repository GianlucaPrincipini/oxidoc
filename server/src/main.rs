use std::net::TcpListener;
use std::io::{Read, Write};
use clap::Parser;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server in ascolto su 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Nuova connessione: {}", stream.peer_addr()?);
                stream.write_all(b"Benvenuto su rusty_db!\n")?;
                // Lettura opzionale dei dati inviati dal client
                let mut buffer = [0; 512];
                let _ = stream.read(&mut buffer)?;
            }
            Err(e) => {
                eprintln!("Errore connessione: {}", e);
            }
        }
    }
    Ok(())
}