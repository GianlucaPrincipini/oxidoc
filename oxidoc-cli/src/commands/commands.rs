use clap::{Args as ClapArgs, Parser, Subcommand};
use clap::error::Error;
use serde::{Deserialize, Serialize};

/// Enum che rappresenta i possibili comandi della CLI.
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CliCommand {
    Status,
    Insert(InsertCommandArgs),
    Get(GetCommandArgs),
    Delete(DeleteCommandArgs),
    CreateCollection(CreateCollectionCommandArgs),
}

impl CliCommand {
    pub fn parse_command(line: &str) -> Result<CliCommand, Error> {
        let args = match shell_words::split(line) {
            Ok(mut args) => {
                args.insert(0, "prog".to_string());
                args
            }
            Err(e) => return Err(Error::raw(clap::error::ErrorKind::InvalidValue, e.to_string())),
        };
        CliCommand::try_parse_from(args)
    }
}

#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CreateCollectionCommandArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub name: String,
}

/// Argomenti per il comando Insert
#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
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
#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct GetCommandArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
}

/// Argomenti per il comando Delete
#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct DeleteCommandArgs {
    /// Nome della collezione
    #[arg(short, long)]
    pub collection: String,
    /// Chiave
    #[arg(short, long)]
    pub key: String,
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
        let command = CliCommand::parse_from([
            "prog", "insert", "-c", "mycoll", "-k", "mykey", "-v", "myval"
        ]);
        assert_eq!(command, CliCommand::Insert(InsertCommandArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
            value: "myval".to_string(),
        }));
    }

    #[test]
    fn parses_get_command_with_args() {
        let command = CliCommand::parse_from([
            "prog", "get", "-c", "mycoll", "-k", "mykey"
        ]);
        assert_eq!(command, CliCommand::Get(GetCommandArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
        }));
    }

    #[test]
    fn parses_delete_command_with_args() {
        let command = CliCommand::parse_from([
            "prog", "delete", "-c", "mycoll", "-k", "mykey"
        ]);
        assert_eq!(command, CliCommand::Delete(DeleteCommandArgs {
            collection: "mycoll".to_string(),
            key: "mykey".to_string(),
        }));
    }

    #[test]
    fn parses_status_command() {
        let command = CliCommand::parse_from(["prog", "status"]);
        assert_eq!(command, CliCommand::Status);
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
