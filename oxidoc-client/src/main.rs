use clap::Parser;
use oxidoc_cli::network::network::read_message;
use oxidoc_client::network::client::connect_to_server;
use oxidoc_client::network::handler::{handle};

const PORT: u16 = 7878;
const ADDRESS: &str = "127.0.0.1";

#[derive(Parser, Debug)]
struct ApplicationArgs {
    #[arg[short, long]]
    server: String,
}

fn main() {
    println!("Welcome to Oxidoc CLI. Type your command or 'exit' to quit.");
    let args: Vec<String> = std::env::args().collect();
    let cli_args = ApplicationArgs::try_parse_from(args);
    let server = match cli_args {
        Err(_) => {
            format!("{ADDRESS}:{PORT}")
        }
        Ok(a) => a.server
    };
    let stream = connect_to_server(server);
    match stream {
        Err(e) => {
            println!("Failed to connect to server: {}", e);
        }
        Ok(s) => {
            let welcome_message = read_message(&s).expect("Failed to read handshake message");
            println!("Server: {}", String::from_utf8_lossy(&welcome_message));
            handle(s)
        }
    }
}
