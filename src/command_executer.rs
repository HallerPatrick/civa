use std::process::Command as SysCommand;
use std::process::ExitStatus;

use super::command_handler::{Command, ExecStrategy};

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ProcessError;

impl Error for ProcessError {}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

pub fn exec_command(command: Command) -> Result<ExitStatus, ProcessError> {
    match command.strategy {
        ExecStrategy::Builtin => {
            if command.command_name == "cd" {
                let child = SysCommand::new(command.command_name.clone())
                    .args(if command.arguments.len() == 0 {
                        vec![String::from(".")]
                    } else {
                        command.arguments
                    })
                    .spawn();

                match child {
                    Ok(mut c) => match c.wait() {
                        Ok(exit_status) => return Ok(exit_status),
                        Err(err) => return Err(ProcessError),
                    },
                    Err(err) => return Err(ProcessError),
                }
            }
            Err(ProcessError)
        }
        _ => Err(ProcessError),
    }
}
