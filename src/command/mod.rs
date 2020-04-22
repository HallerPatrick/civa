pub mod error;
pub mod executer;
pub mod handler;

#[derive(Debug, PartialEq)]
pub enum ExecStrategy {
    Builtin,
    // SpecialBuiltin,
    // Unspecific,
    // ShellFunction,
    // OtherUtilities,
    PathCommand,
    Undefined,
    Pipe(PipeType)
}

#[derive(Debug, Clone, PartialEq)]
pub enum PipeType {
    OnlyOutput,
    OnlyInput,
    OutAndInput
}

#[derive(Debug, PartialEq)]
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
