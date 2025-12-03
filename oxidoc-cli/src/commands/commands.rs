use clap::error::Error;
use clap::error::ErrorKind::InvalidValue;
use clap::{Args as ClapArgs, Parser};
use serde::{Deserialize, Serialize};

/// CLI Commands
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CliCommand {
    Status,
    Insert(InsertCommandArgs),
    Get(GetCommandArgs),
    Delete(DeleteCommandArgs),
    CreateCollection(CreateCollectionCommandArgs),
    DeleteCollection(CreateCollectionCommandArgs),
}

impl CliCommand {
    pub fn parse_command(line: &str) -> Result<CliCommand, Error> {
        let mut args = shell_words::split(line)
            .map_err(|e| Error::raw(InvalidValue, e.to_string()))?;
        args.insert(0, "prog".to_string());
        CliCommand::try_parse_from(args)
    }
}

#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CreateCollectionCommandArgs {
    #[arg(short, long)]
    pub name: String,
}

#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct DeleteCollectionCommandArgs {
    #[arg(short, long)]
    pub name: String,
}

#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct InsertCommandArgs {
    #[arg(short, long)]
    pub collection: String,
    
    #[arg(short, long)]
    pub key: String,
    
    #[arg(short, long)]
    pub value: String,
}

#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct GetCommandArgs {
    #[arg(short, long)]
    pub collection: String,
    
    #[arg(short, long)]
    pub key: String,
}

#[derive(ClapArgs, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct DeleteCommandArgs {
    #[arg(short, long)]
    pub collection: String,
    
    #[arg(short, long)]
    pub key: String,
}


impl CliCommand {
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
