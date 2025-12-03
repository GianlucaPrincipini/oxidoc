use clap::{Parser, Subcommand, Args as ClapArgs};
use serde::{Deserialize, Serialize};

/// Enum che rappresenta i possibili comandi della CLI.
#[derive(Subcommand, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CliCommand {
    /// Mostra lo stato del database
    Status,
    /// Inserisce una chiave/valore in una collezione
    Insert(InsertCommandArgs),
    /// Recupera un valore da una collezione
    Get(GetCommandArgs),
    /// Elimina una chiave da una collezione
    Delete(DeleteCommandArgs),
}

/// Argomenti per il comando Insert
#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsertCommandArgs {
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
#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetCommandArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
}

/// Argomenti per il comando Delete
#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteCommandArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
}

/// Struttura principale che rappresenta la CLI di oxidoc.
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct OxidocCli {
    #[command(subcommand)]
    pub command: CliCommand,
}

impl CliCommand {
    /// Serializza la struct in JSON e restituisce i byte risultanti.
    pub fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Serialization failed")
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).expect("Deserialization failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;


    #[test]
    fn parses_insert_command_with_all_args() {
        let cli = OxidocCli::parse_from([
            "prog", "insert", "-c", "mycoll", "-k", "mykey", "-v", "myval"
        ]);
        assert_eq!(cli.command, CliCommand::Insert(InsertCommandArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
            value: "myval".to_string(),
        }));
    }

    #[test]
    fn parses_get_command_with_args() {
        let cli = OxidocCli::parse_from([
            "prog", "get", "-c", "mycoll", "-k", "mykey"
        ]);
        assert_eq!(cli.command, CliCommand::Get(GetCommandArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
        }));
    }

    #[test]
    fn parses_delete_command_with_args() {
        let cli = OxidocCli::parse_from([
            "prog", "delete", "-c", "mycoll", "-k", "mykey"
        ]);
        assert_eq!(cli.command, CliCommand::Delete(DeleteCommandArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
        }));
    }

    #[test]
    fn parses_status_command() {
        let cli = OxidocCli::parse_from(["prog", "status"]);
        assert_eq!(cli.command, CliCommand::Status);
    }

    #[test]
    fn serializes_and_deserializes_cli_command() {
        let original = CliCommand::Insert(InsertCommandArgs {
            collection: "test_collection".to_string(),
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        });
        let bytes = original.as_bytes();
        let deserialized = CliCommand::from_bytes(&bytes);
        assert_eq!(original, deserialized);
    }

}
