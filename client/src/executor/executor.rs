
use crate::utils::cli::Command;
pub trait CommandExecutor {
    fn execute(&self, command: Command) -> String;
}

pub struct CommandExecutorFactory;
impl CommandExecutorFactory {
    pub fn get_executor(ex_type: String) -> Box<dyn CommandExecutor> {
        match ex_type.as_str() {
            // Here you can add different executor types in the future
            "dummy" => {return Box::new(DummyCommandExecutor);},
            _ => {panic!("Unknown executor type: {}", ex_type);}
        }
        Box::new(DummyCommandExecutor)
    }
}

pub struct DummyCommandExecutor;

impl CommandExecutor for DummyCommandExecutor{
    fn execute(&self, command: Command) -> String {
        match command {
            Command::Start => "Executor started.".to_string(),
            Command::Stop => "Executor stopped.".to_string(),
            Command::Insert(args) => format!(
                "Inserted key '{}' with value '{}' into collection '{}'.",
                args.key, args.value, args.collection
            ),
            Command::Get(args) => format!(
                "Retrieved value for key '{}' from collection '{}'.",
                args.key, args.collection
            ),
            Command::Delete(args) => format!(
                "Deleted key '{}' from collection '{}'.",
                args.key, args.collection
            ),
            Command::Status => "Executor status: Running.".to_string(),
            
        }
    }
}