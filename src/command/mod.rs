pub mod error;
pub mod executer;
pub mod handler;

#[derive(Debug)]
pub enum ExecStrategy {
    Builtin,
    // SpecialBuiltin,
    // Unspecific,
    // ShellFunction,
    // OtherUtilities,
    PathCommand,
    Undefined,
}

#[derive(Debug)]
pub struct Command {
    pub command_name: String,
    pub arguments: Vec<String>,
    pub strategy: ExecStrategy,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            command_name: String::new(),
            arguments: Vec::new(),
            strategy: ExecStrategy::Undefined,
        }
    }
}
