use std::process::Command as SysCommand;

use log::error;

use super::builtins::cd;
use super::builtins::error::ProcessError;
use super::builtins::exit_status::ExitStatus;
use super::command_handler::{Command, ExecStrategy};

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

fn exec_command(command: Command) -> Result<ExitStatus, ProcessError> {
    match command.strategy {
        ExecStrategy::Builtin => match command.command_name.as_str() {
            "cd" => return cd::cd(command.arguments.first()),
            _ => {
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
                        Err(_) => return Err(ProcessError),
                    },
                    Err(_) => return Err(ProcessError),
                }
            }
        },
        _ => Err(ProcessError),
    }
}
