pub mod error;
pub mod executer;
pub mod handler;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExecStrategy {
    Builtin,
    // SpecialBuiltin,
    // Unspecific,
    // ShellFunction,
    // OtherUtilities,
    PathCommand,
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PipeType {
    ReceivesInput,
    PassesOutput,
    OutAndInput,
    Undefined
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub command_name: String,
    pub arguments: Vec<String>,
    pub strategy: ExecStrategy,
    pub pipe_type: PipeType
}

impl Default for Command {
    fn default() -> Self {
        Self {
            command_name: String::new(),
            arguments: Vec::new(),
            strategy: ExecStrategy::Undefined,
            pipe_type: PipeType::Undefined
        }
    }
}
