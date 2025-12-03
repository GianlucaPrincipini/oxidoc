use oxidoc_cli::commands::commands::{OxidocCli};
use oxidoc_cli::commands::executor::CommandExecutorFactory;
use clap::Parser;
use std::{io, mem};
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::Serialize;

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
        let args = input.split_whitespace().collect::<Vec<_>>();
        match OxidocCli::try_parse_from(args) {
            Ok(cmd) => {
                // CommandExecutorFactory::get_executor("dummy".to_string()).execute(cmd.command);
                stream.write_all(cmd.command.as_bytes().as_slice()).unwrap();
                stream.read(&mut [0; 64]).unwrap();
            }
            Err(e) => println!("Invalid command: {}", e),
        }
    }
}