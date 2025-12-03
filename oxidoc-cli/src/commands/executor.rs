use std::process::Command;
use crate::commands::commands::{CliCommand};
pub trait CommandHandler {
    fn execute(&self, command: CliCommand) -> String;
}

pub struct CommandExecutorFactory;
impl CommandExecutorFactory {
    pub fn get_executor(ex_type: String) -> Box<dyn CommandHandler> {
        match ex_type.as_str() {
            // Here you can add different executor types in the future
            "dummy" => {return Box::new(DummyCommandExecutor);},
            _ => {panic!("Unknown executor type: {}", ex_type);}
        }
        Box::new(DummyCommandExecutor)
    }
}

pub struct DummyCommandExecutor;

impl CommandHandler for DummyCommandExecutor{
    fn execute(&self, command: CliCommand) -> String {
        match command {
            CliCommand::Insert(args) => format!(
                "Inserted key '{}' with value '{}' into collection '{}'.",
                args.key, args.value, args.collection
            ),
            CliCommand::Get(args) => format!(
                "Retrieved value for key '{}' from collection '{}'.",
                args.key, args.collection
            ),
            CliCommand::Delete(args) => format!(
                "Deleted key '{}' from collection '{}'.",
                args.key, args.collection
            ),
            CliCommand::Status => "Executor status: Running.".to_string(),

        }
    }
}