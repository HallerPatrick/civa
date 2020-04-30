use std::io::Read;
use std::process::Command as SysCommand;
use std::process::Stdio;

use log::{error, info};

use super::error::CommandError;
use crate::builtins::executer;
use crate::builtins::exit_status::ExitStatus;
use crate::command::{Command, ExecStrategy, PipeType};

//
// Depending on using pipes or just the sequential delimiter
// We have to capture the stdout out and pipe it into
// the stdin of the next command.
//
// If it just a sequential, then throw exit code etc away...for now
//
//
//TODO: Solve Bug where pipeline output is not passed to stdout
pub fn exec_sequentially(commands: &mut Vec<Command>) -> ExitStatus {
    let mut current_status: ExitStatus = ExitStatus { code: -1 };

    while !commands.is_empty() {
        // Peek
        let command = commands.first().unwrap();

        info!("Execute command {:?}", command);
        match command.pipe_type {
            PipeType::Undefined => match exec_command(commands.pop().unwrap()) {
                Ok(exit_status) => current_status = exit_status,
                Err(err) => {
                    error!("{}", err);
                    println!("{}", err.message);
                }
            },
            _ => {
                execute_pipe(commands);
            }
        }
    }

    current_status
}

fn exec_command(command: Command) -> Result<ExitStatus, CommandError> {
    match command.strategy {
        ExecStrategy::Builtin => match executer::executor(command) {
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
                    Ok(exit_status) => Ok(ExitStatus {
                        code: exit_status.code().unwrap(),
                    }),
                    Err(_) => Err(CommandError {
                        kind: String::from("process"),
                        message: String::from("Could not get exit code of process"),
                    }),
                },
                Err(_) => Err(CommandError {
                    kind: String::from("process"),
                    message: format!("Could not find command {}", command.command_name),
                }),
            }
        }
        _ => Err(CommandError {
            kind: String::from("exec_command"),
            message: String::from("Could not determine execution strategy"),
        }),
    }
}

fn execute_pipe(commands: &mut Vec<Command>) {
    let mut output: Vec<u8> = Vec::new();
    pipe_consumer(commands, None, &mut output);
    info!("Ouput: {}", String::from_utf8(output).unwrap());
}

#[allow(unused_must_use)]
fn pipe_consumer(
    vec: &mut Vec<Command>,
    out: Option<std::process::ChildStdout>,
    output: &mut Vec<u8>,
) {
    if vec.is_empty() {
        out.unwrap().read(output);
        return;
    }

    let cmd = vec.pop().unwrap();

    if cmd.pipe_type == PipeType::Undefined {
        return;
    }

    match out {
        Some(stdout) => {
            let current_command = SysCommand::new(cmd.command_name)
                .args(cmd.arguments)
                .stdout(Stdio::piped())
                .stdin(stdout)
                .spawn()
                .expect("Failure");

            // print!("{:?}", current_command);
            pipe_consumer(vec, current_command.stdout, output);
        }
        None => {
            let current_command = SysCommand::new(cmd.command_name)
                .args(cmd.arguments)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failure");

            // print!("{:?}", String::from_utf8(current_command.stdin));

            pipe_consumer(vec, current_command.stdout, output);
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::fs;

    fn helper_get_files_in_dir(path: &str) -> Vec<String> {
        let paths = fs::read_dir(path).unwrap();

        let mut ps: Vec<String> = Vec::new();

        for p in paths {
            let mut pa = String::from(p.unwrap().path().as_os_str().to_str().unwrap());
            pa = pa.replace("./", "");

            if pa != "" {
                ps.push(pa);
            }
        }

        ps.sort();
        ps
    }

    #[test]
    fn pipes() {
        let cmd = SysCommand::new("ls")
            .arg("-a")
            .arg(".")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed command");

        let cmd2 = SysCommand::new("cat")
            .stdin(cmd.stdout.unwrap())
            .output()
            .expect("Failure cat");

        let expected_result = helper_get_files_in_dir(".");
        let mut result: Vec<String> = String::from_utf8(cmd2.stdout)
            .unwrap()
            .split("\n")
            .map(|s| String::from(s))
            .filter(|s| s != &String::from("."))
            .filter(|s| s != &String::from(".."))
            .filter(|s| s != &String::from(""))
            .collect();

        result.sort();

        assert_eq!(expected_result, result);
    }

    #[test]
    fn pipe() {
        let cmds = vec![
            Command {
                command_name: String::from("ls"),
                arguments: vec![String::from("-a"), String::from(".")],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::PassesOutput,
            },
            Command {
                command_name: String::from("cat"),
                arguments: vec![],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::ReceivesInput,
            },
        ];

        let expected_result = helper_get_files_in_dir(".");

        // let out = execute_pipe(cmds);
        // print!("{:?}", out);
    }

    #[test]
    fn pipe_ls() {
        let cmds = vec![
            Command {
                command_name: String::from("ls"),
                arguments: vec![String::from("-a"), String::from(".")],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::PassesOutput,
            },
            Command {
                command_name: String::from("ls"),
                arguments: vec![],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::ReceivesInput,
            },
        ];

        let expected_result = helper_get_files_in_dir(".");

        // let out = execute_pipe(cmds);
    }
}
