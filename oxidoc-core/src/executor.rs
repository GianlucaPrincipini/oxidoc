use crate::database::Database;
use crate::response::Response;
use oxidoc_cli::commands::commands::CliCommand;
use std::io::Error;

pub fn execute_command(command: CliCommand, db: &mut Database) -> Result<Response, Error> {
    match command {
        CliCommand::Insert(args) => {
            let clone = args.clone();
            let val = serde_json::from_str(&args.value)?;
            db.put(args.collection, args.key, val);
            Ok(Response::Success(format!("Insert success. {:?}", clone)))
        }
        CliCommand::Get(args) => Ok(Response::Doc(db.get(&args.collection, &args.key).cloned())),
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
    use crate::database::Database;
    use oxidoc_cli::commands::commands::{DeleteCommandArgs, GetCommandArgs, InsertCommandArgs};
    use crate::collection::Document;

    #[test]
    fn insert_returns_success_response() {
        let mut db = Database::initialize();
        let insert_cmd = CliCommand::Insert(InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        let response = execute_command(insert_cmd, &mut db);
        assert!(matches!(response, Ok(Response::Success(_))));
    }

    #[test]
    fn insert_success_message_is_correct() {
        let mut db = Database::initialize();
        let insert_cmd = CliCommand::Insert(InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        let response = execute_command(insert_cmd, &mut db);
        let expected = format!(
            "Insert success. {:?}",
            InsertCommandArgs {
                collection: "test_coll".to_string(),
                key: "key1".to_string(),
                value: r#"{"field":"value1"}"#.to_string(),
            }
        );
        assert_eq!(response.unwrap(), Response::Success(expected));
    }

    #[test]
    fn get_returns_doc_with_value_after_insert() {
        let mut db = Database::initialize();
        // Inserisci prima
        let insert_cmd = CliCommand::Insert(InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        execute_command(insert_cmd, &mut db).unwrap();
        // Poi recupera
        let get_cmd = CliCommand::Get(GetCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = execute_command(get_cmd, &mut db);
        assert!(matches!(response, Ok(Response::Doc(Some(_)))));
    }

    #[test]
    fn get_returns_expected_document() {
        let mut db = Database::initialize();
        // Inserisci prima
        let insert_cmd = CliCommand::Insert(InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        execute_command(insert_cmd, &mut db).unwrap();
        // Poi recupera
        let get_cmd = CliCommand::Get(GetCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = execute_command(get_cmd, &mut db);
        let expected: Document = serde_json::from_str(r#"{"field":"value1"}"#).unwrap();
        match response {
            Ok(Response::Doc(Some(doc))) => assert_eq!(doc, expected),
            _ => panic!("Expected Doc response with value for Get"),
        }
    }

    #[test]
    fn delete_returns_ack() {
        let mut db = Database::initialize();
        // Inserisci prima
        let insert_cmd = CliCommand::Insert(InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        execute_command(insert_cmd, &mut db).unwrap();
        // Cancella
        let delete_cmd = CliCommand::Delete(DeleteCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = execute_command(delete_cmd, &mut db);
        assert!(matches!(response, Ok(Response::Ack)));
    }

    #[test]
    fn get_returns_none_after_delete() {
        let mut db = Database::initialize();
        // Inserisci prima
        let insert_cmd = CliCommand::Insert(InsertCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
            value: r#"{"field":"value1"}"#.to_string(),
        });
        execute_command(insert_cmd, &mut db).unwrap();
        // Cancella
        let delete_cmd = CliCommand::Delete(DeleteCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        execute_command(delete_cmd, &mut db).unwrap();
        // Recupera
        let get_cmd = CliCommand::Get(GetCommandArgs {
            collection: "test_coll".to_string(),
            key: "key1".to_string(),
        });
        let response = execute_command(get_cmd, &mut db);
        assert!(matches!(response, Ok(Response::Doc(None))));
    }
}
