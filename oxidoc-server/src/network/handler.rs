use std::error::Error;
use oxidoc_cli::commands::commands::CliCommand;
use oxidoc_cli::network::network::{read_message, write_message};
use oxidoc_core::handler::execute_command;
use std::net::TcpStream;
use oxidoc_core::handler::Response::Failure;

pub enum HandlerType {
    Server,
    Client,
}

pub fn handle_stream(stream: TcpStream) -> () {
    println!("New connection: {}", stream.peer_addr().expect("Unable to get peer address"));
    let message: &[u8] = b"Welcome to oxidoc!\n";
    write_message(&stream, message).expect("Failed to send welcome message");
    let mut db = oxidoc_core::database::Database::initialize();
    loop {
        let buffer = match read_message(&stream) {
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
            Ok(buffer) => buffer,
        };
        match (String::from_utf8_lossy(&buffer.to_vec())).as_ref() {
            "exit" | "quit" => {
                println!("Client requested to close the connection.");
                break;
            }
            _ => {
                let command = CliCommand::from_bytes(&buffer);
                println!("Received data: {:?}", command);
                match execute_command(command, &mut db) {
                    Ok(response) => {
                        write_message(&stream, response.as_bytes().as_ref()).expect("Failed to send response");
                    }
                    Err(e) => {
                        let error_message = format!("Error processing command: {}", e);
                        let response = Failure(error_message);
                        write_message(&stream, response.as_bytes().as_ref()).expect("Failed to send error message");
                    }
                }
            }
        }
    }
}