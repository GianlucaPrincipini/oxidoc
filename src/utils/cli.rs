use clap::{Parser, Subcommand, Args as ClapArgs};

/// Enum che rappresenta i possibili comandi della CLI.
#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Command {
    /// Avvia il database
    Start,
    /// Ferma il database
    Stop,
    /// Mostra lo stato del database
    Status,
    /// Inserisce una chiave/valore in una collezione
    Insert(InsertArgs),
    /// Recupera un valore da una collezione
    Get(GetArgs),
    /// Elimina una chiave da una collezione
    Delete(DeleteArgs),
}

/// Argomenti per il comando Insert
#[derive(ClapArgs, Debug, PartialEq, Eq)]
pub struct InsertArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
    /// Valore
    #[arg(short, long)]
    pub value: String,
}

/// Argomenti per il comando Get
#[derive(ClapArgs, Debug, PartialEq, Eq)]
pub struct GetArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
}

/// Argomenti per il comando Delete
#[derive(ClapArgs, Debug, PartialEq, Eq)]
pub struct DeleteArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
}

/// Struttura principale che rappresenta la CLI di rusty_db.
#[derive(Parser, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_start_command() {
        let cli = Cli::parse_from(["prog", "start"]);
        assert_eq!(cli.command, Command::Start);
    }

    #[test]
    fn parses_insert_command_with_all_args() {
        let cli = Cli::parse_from([
            "prog", "insert", "-c", "mycoll", "-k", "mykey", "-v", "myval"
        ]);
        assert_eq!(cli.command, Command::Insert(InsertArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
            value: "myval".to_string(),
        }));
    }

    #[test]
    fn parses_get_command_with_args() {
        let cli = Cli::parse_from([
            "prog", "get", "-c", "mycoll", "-k", "mykey"
        ]);
        assert_eq!(cli.command, Command::Get(GetArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
        }));
    }

    #[test]
    fn parses_delete_command_with_args() {
        let cli = Cli::parse_from([
            "prog", "delete", "-c", "mycoll", "-k", "mykey"
        ]);
        assert_eq!(cli.command, Command::Delete(DeleteArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
        }));
    }

    #[test]
    fn parses_status_command() {
        let cli = Cli::parse_from(["prog", "status"]);
        assert_eq!(cli.command, Command::Status);
    }

    #[test]
    fn parses_stop_command() {
        let cli = Cli::parse_from(["prog", "stop"]);
        assert_eq!(cli.command, Command::Stop);
    }
}
