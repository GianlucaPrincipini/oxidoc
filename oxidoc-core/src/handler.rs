use crate::collection::Document;
use crate::database::Database;
use oxidoc_cli::commands::commands::CliCommand;
use std::fmt::Display;
use std::io::Error;

#[derive(Debug)]
pub enum Response<'a> {
    Success(String),
    Doc(Option<&'a Document>),
    Failure(String),
    Ack,
}

impl Display for Response<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Success(msg) => write!(f, "Success: {}", msg),
            Response::Doc(Some(doc)) => write!(f, "Document: {:?}", doc),
            Response::Doc(None) => write!(f, "Document: None"),
            Response::Failure(msg) => write!(f, "Server responded with failure: {}", msg),
            Response::Ack => write!(f, "Acknowledged"),
        }
    }
}

impl Response<'_> {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Response::Success(msg) => msg.as_bytes().to_vec(),
            Response::Doc(Some(doc)) => serde_json::to_string(doc).expect("1").as_bytes().to_vec(),
            Response::Doc(None) => b"2".to_vec(),
            Response::Failure(msg) => {
                let mut r = msg.as_bytes().to_vec();
                r.insert(0, b'!');
                r
            }
            Response::Ack => b"ACK".to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let s = String::from_utf8_lossy(bytes);
        if s == "ACK" {
            Response::Ack
        } else if s == "2" {
            Response::Doc(None)
        } else if s.starts_with('{') && s.ends_with('}') {
            let doc: Document =
                serde_json::from_str(&s).expect("Deserialization failed for Document.");
            Response::Doc(Some(Box::leak(Box::new(doc))))
        } else if s.starts_with("!") {
            Response::Failure(s[1..].to_string())
        } else {
            Response::Success(s.to_string())
        }
    }
}

pub fn database_handler(command: CliCommand, db: &'_ mut Database) -> Result<Response<'_>, Error> {
    match command {
        CliCommand::Insert(args) => {
            let clone = args.clone();
            match serde_json::from_str(&args.value) {
                Ok(val) => {
                    db.put(args.collection, args.key, val);
                    Ok(Response::Success(format!("Insert success. {:?}", clone)))
                }
                Err(e) => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid JSON value: {}", e),
                    ));
                }
            }
        }
        CliCommand::Get(args) => Ok(Response::Doc(db.get(&args.collection, &args.key))),
        CliCommand::Delete(args) => {
            db.delete(args.collection, args.key);
            Ok(Response::Ack)
        }
        CliCommand::Status => Ok(Response::Success(
            "Database status: Operational.".to_string(),
        )),
        CliCommand::DeleteCollection(args) => {
            let clone = args.clone();
            db.delete_collection(&args.name);
            Ok(Response::Success(format!(
                "Collection {:?} deleted.",
                clone
            )))
        },
        CliCommand::CreateCollection(args) => {
            let clone = args.clone();
            db.create_collection(args.name);
            Ok(Response::Success(format!(
                "Collection {:?} created.",
                clone
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_handler() {
        let mut db = Database::initialize();

        // Test Insert
        let insert_cmd = CliCommand::Insert(oxidoc_cli::commands::commands::InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        let response = database_handler(insert_cmd, &mut db);
        match response {
            Ok(Response::Success(value)) => {
                println!("{value}");
                let str = format!(
                    "Insert success. {:?}",
                    oxidoc_cli::commands::commands::InsertCommandArgs {
                        collection: "test_coll".to_string(),
                        key: "key1".to_string(),
                        value: r#"{"field":"value1"}"#.to_string(),
                    }
                );
                assert_eq!(value, str)
            }
            _ => panic!("Expected Success response for Insert"),
        }

        // Test Get
        let get_cmd = CliCommand::Get(oxidoc_cli::commands::commands::GetCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = database_handler(get_cmd, &mut db);
        match response {
            Ok(Response::Doc(Some(doc))) => {
                let expected: Document = serde_json::from_str(r#"{"field":"value1"}"#).unwrap();
                assert_eq!(doc, &expected);
            }
            _ => panic!("Expected Doc response with value for Get"),
        }

        // Test Delete
        let delete_cmd = CliCommand::Delete(oxidoc_cli::commands::commands::DeleteCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = database_handler(delete_cmd, &mut db);
        match response {
            Ok(Response::Ack) => (),
            _ => panic!("Expected Empty response for Delete"),
        }

        // Verify Deletion
        let get_cmd = CliCommand::Get(oxidoc_cli::commands::commands::GetCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = database_handler(get_cmd, &mut db);
        match response {
            Ok(Response::Doc(None)) => (),
            _ => panic!("Expected Doc response with None for deleted key"),
        }
    }
}
