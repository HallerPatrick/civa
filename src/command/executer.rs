use std::process::Command as SysCommand;

use log::{error, info};

use super::error::CommandError;
use crate::builtins::executer;
use crate::builtins::exit_status::ExitStatus;
use crate::command::{Command, ExecStrategy};

pub fn exec_sequentially(commands: Vec<Command>) -> ExitStatus {
    let mut current_status: ExitStatus = ExitStatus { code: -1 };

    for command in commands {
        match exec_command(command) {
            Ok(exit_status) => current_status = exit_status,
            Err(err) => {
                error!("{}", err);
            }
        }
    }

    current_status
}

fn exec_command(command: Command) -> Result<ExitStatus, CommandError> {
    match command.strategy {
        ExecStrategy::Builtin => match executer::executer(command) {
            Ok(exit_status) => Ok(exit_status),
            Err(err) => Err(CommandError::from(err)),
        },
        ExecStrategy::PathCommand => {
            info!("Calling command: {}", command.command_name);
            info!("With arguments: {:?}", command.arguments);
            let child = SysCommand::new(command.command_name.clone())
                .args(command.arguments)
                .spawn();
            match child {
                Ok(mut c) => match c.wait() {
                    Ok(exit_status) => {
                        return Ok(ExitStatus {
                            code: exit_status.code().unwrap(),
                        })
                    }
                    Err(_) => Err(CommandError {
                        kind: String::from("process"),
                        message: String::from("Could not get exit code of process"),
                    }),
                },
                Err(_) => {
                    return Err(CommandError {
                        kind: String::from("process"),
                        message: String::from("Could not wait on process to finish"),
                    })
                }
            }
        }
        _ => Err(CommandError {
            kind: String::from("exec_command"),
            message: String::from("Could not determine execution strategy"),
        }),
    }
}
