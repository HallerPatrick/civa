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
    AbsolutePathCommand,
    Undefined,
    SlashCommand,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PipeType {
    ReceivesInput,
    PassesOutput,
    OutAndInput,
    Undefined,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub command_name: String,
    pub arguments: Vec<String>,
    pub strategy: ExecStrategy,
    pub pipe_type: PipeType,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Self {
            command_name: self.command_name.clone(),
            arguments: self.arguments.clone(),
            strategy: self.strategy.clone(),
            pipe_type: self.pipe_type.clone(),
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            command_name: String::new(),
            arguments: Vec::new(),
            strategy: ExecStrategy::Undefined,
            pipe_type: PipeType::Undefined,
        }
    }
}
