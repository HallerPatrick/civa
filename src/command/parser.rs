use std::process::Command as SysCommand;
use std::process::Stdio;

enum TokenType {
    COMMAND_NAME,
    ARGUMENT,
    COMMAND_DELIMITER,
    PIPE,
    INPUTREDIRECTION,
    OUTPUTREDIRECTION,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExecStrategy {
    Builtin,
    PathCommand,
    AbsolutePathCommand,
    Undefined,
    SlashCommand,
    ArithmeticExpression,
}

struct Command {
    stdin: Stdio,
    stdout: Stdio,
    stderr: Stdio,
    command_name: String,
    strategy: ExecStrategy,
    arguments: Vec<String>,
}
