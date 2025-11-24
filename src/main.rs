use clap::Parser;
use std::io::{self, Write};
use rusty_db::executor::executor::CommandExecutorFactory;
use rusty_db::utils::cli::Cli;

fn main() {
    loop {
        print!("rusty_db> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Errore nella lettura dell'input");
            continue;
        }
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Uscita dal programma.");
            break;
        }
        // Simula la suddivisione degli argomenti come da CLI
        let args = std::iter::once("rusty_db").chain(input.split_whitespace());
        match Cli::try_parse_from(args) {
            Ok(cli) => {
                let res = CommandExecutorFactory::get_executor(String::from("dummy")).execute(cli.command);
                println!("{}", res);
            }
            ,
            Err(e) => println!("Errore di parsing: {}", e),
        }
    }
}
