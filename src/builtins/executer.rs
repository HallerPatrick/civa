use crate::command::Command;

use super::alias;
use super::cd;
use super::error::BuiltinError;
use super::exit_status::ExitStatus;
use super::penv;
use super::BUILTIN_NAMES;

use crate::config::ContextManager;

pub fn executor(command: Command, ctx: &ContextManager) -> Result<ExitStatus, BuiltinError> {
    if BUILTIN_NAMES.contains(&command.command_name.as_str()) {
        match command.command_name.as_str() {
            "cd" => cd::cd(command.arguments.first()),
            ":q" => std::process::exit(0),
            "penv" => penv::penv(command.arguments.first().unwrap_or(&String::new())),
            "alias" => alias::alias(command.arguments.clone(), ctx),
            other => Err(BuiltinError {
                kind: String::from("builtins"),
                message: format!("Could not find builtin '{}'", other),
            }),
        }
    } else {
        Err(BuiltinError {
            kind: String::from("builtins"),
            message: format!("Could not find builtin '{}'", command.command_name),
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_executor_no_builtin_found() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from("test");

        let ctx = ContextManager::init();

        let result = executor(cmd, &ctx);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_executor_builtin_found() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from("cd");

        let ctx = ContextManager::init();
        let result = executor(cmd, &ctx);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn build_found_with_args() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from("penv");
        cmd.arguments = vec![String::from("PATH")];

        let ctx = ContextManager::init();
        let result = executor(cmd, &ctx);

        assert_eq!(result.is_ok(), true);

        assert_eq!(result.ok(), Some(ExitStatus { code: 1 }));
    }

    #[test]
    fn build_found_failing() {
        let mut cmd: Command = Command::default();
        cmd.command_name = String::from("penv");

        let ctx = ContextManager::init();
        let result = executor(cmd, &ctx);

        assert!(result.is_err());
    }
}
