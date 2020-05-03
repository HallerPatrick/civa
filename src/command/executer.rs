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
        ExecStrategy::PathCommand
        | ExecStrategy::SlashCommand
        | ExecStrategy::AbsolutePathCommand => {
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
    pipe_consumer(commands, None);
}

// TODO: Fix Bug
// BUG: When pipe ends in a 'cat' command, sometimes the output is not finished corretly
// and rustyline cannot find the location to place the new cursor and command bar is not displayed
fn pipe_consumer(commands: &mut Vec<Command>, stdout: Option<std::process::ChildStdout>) {
    if commands.is_empty() {
        return;
    }

    let cmd = commands.pop().unwrap();

    // If the end of pipe is reached construct a non pipe command which takes
    // only only previous stdout
    if commands.is_empty() || commands.first().unwrap().pipe_type == PipeType::Undefined {
        info!("Called last");
        let process = SysCommand::new(cmd.command_name)
            .args(cmd.arguments)
            .stdin(stdout.unwrap())
            .spawn();

        match process {
            Ok(mut c) => match c.wait() {
                Ok(_) => {}
                Err(_) => info!("Could not get exit code of process"),
            },
            Err(_) => info!("Could not find command "),
        }

        return;
    }

    match stdout {
        // The is stdout of a previous command, pipe it in new command
        Some(prev_stdout) => {
            info!("Called middle");
            let process = SysCommand::new(cmd.command_name)
                .args(cmd.arguments)
                .stdin(prev_stdout)
                .stdout(Stdio::piped())
                .spawn()
                .expect("MEEEEH");
            pipe_consumer(commands, process.stdout);
        }
        // There is no previous stdout -> First called command of pipe
        None => {
            info!("Called first");
            let process = SysCommand::new(cmd.command_name)
                .args(cmd.arguments)
                .stdout(Stdio::piped())
                .spawn()
                .expect("MEEEEH");

            pipe_consumer(commands, process.stdout);
        }
    }
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
        // There are three types of commands we have to construct
        //
        // 1. A Command with a pipe for stdout
        let cmd = SysCommand::new("ls")
            .arg("-a")
            .arg(".")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed command");

        // 2. A command which takes the first commands stdout as stding and a new pipe for stdout
        let cmd2 = SysCommand::new("cat")
            .stdin(cmd.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failure cat");

        // 3. A command which takes the second commands stdout as stdin and produces the stdout to
        //    parent stdout
        let cmd3 = SysCommand::new("cat")
            .stdin(cmd2.stdout.unwrap())
            .output()
            .expect("Failure cat");

        let expected_result = helper_get_files_in_dir(".");
        let mut result: Vec<String> = String::from_utf8(cmd3.stdout)
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
    fn pipe_consumer_test() {
        let mut cmds = vec![
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

        // let expected_result = helper_get_files_in_dir(".");

        pipe_consumer(&mut cmds, None);
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
