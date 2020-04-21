use std;

use super::cd;
use super::error::ProcessError;
use super::exit_status::ExitStatus;
use super::BUILTIN_NAMES;
use crate::command_handler::Command;

pub fn executer(command: Command) -> Result<ExitStatus, ProcessError> {
    if BUILTIN_NAMES.contains(&command.command_name.as_str()) {
        match command.command_name.as_str() {
            "cd" => cd::cd(command.arguments.first()),
            ":q" => std::process::exit(0),
            other => Err(ProcessError {
                kind: String::from("builtins"),
                message: String::from(format!("Could not find builtin '{}'", other)),
            }),
        }
    } else {
        Err(ProcessError {
            kind: String::from("builtins"),
            message: String::from(format!("Could not find builtin '{}'", command.command_name)),
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_executer_no_builtin_found() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from("test");

        let result = executer(cmd);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_executer_builtin_found() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from("cd");

        let result = executer(cmd);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_executer_quit() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from(":q");

        let result = executer(cmd);

        assert_eq!(result.is_ok(), true);

        assert_eq!(result.ok(), Some(ExitStatus { code: 0 }));
    }
}
