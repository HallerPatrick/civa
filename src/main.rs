mod builtins;
mod command_executer;
mod command_handler;

use builtins::cd;
use command_handler::handle_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::process::Command;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let commands: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

                handle_command(line.as_str());

                // Check first for predefined commands
                if commands[0] == "cd" {
                    match cd::cd(commands[1]) {
                        Ok(()) => {}
                        Err(err) => println!("{}", err),
                    }
                } else if commands[0] == ":q" {
                    break;
                } else {
                    let child = Command::new(commands[0]).args(&commands[1..]).spawn();

                    match child {
                        Ok(mut c) => match c.wait() {
                            Ok(exit_status) => println!("Exist Status: {}", exit_status),
                            Err(err) => {
                                panic!("Could not wait for child process to finish: {}", err);
                            }
                        },
                        Err(err) => println!("Error: {}", err),
                    }
                }
            }
            // "Soft Reset" the shell

            // Ctrl-C
            Err(ReadlineError::Interrupted) => continue,

            // Ctrl-D
            Err(ReadlineError::Eof) => continue,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
