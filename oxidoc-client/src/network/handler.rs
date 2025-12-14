use oxidoc_cli::commands::commands::CliCommand;
use oxidoc_cli::network::network::{read_message, write_message};
use std::io;
use std::io::Write;
use std::net::TcpStream;
use oxidoc_core::response::Response;

pub struct CommandHandler;
impl CommandHandler {
    pub fn handle_command(&self, command: &str) {
        println!("Handling command: {}", command);
    }
}

pub fn handle(mut stream: TcpStream) {
    loop {
        print!("oxidoc> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Please try again.");
            continue;
        }
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Exiting CLI. Goodbye!");
            stream.flush().unwrap();
            break;
        }
        match CliCommand::parse_command(input) {
            Ok(command) => {
                let bytes = command.as_bytes();
                write_message(&stream, &bytes).expect("Error writing to stream.");
                
                let response  = Response::from_bytes(&*read_message(&stream).unwrap());
                handle_response(response);
            }
            Err(e) => println!("Invalid command: {}", e),
        }
    }
}

fn handle_response(p0: Response) {
    match p0 {
        Response::Success(msg) => println!("Success: {}", msg),
        Response::Doc(doc) => println!("Document: {}", doc.unwrap().to_string()),
        Response::Ack => println!("Acknowledged."),
        Response::Failure(err) => println!("Error: {}", err),
    }
}
