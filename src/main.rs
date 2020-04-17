mod builtins;

use ctrlc;
use rustyline::error::ReadlineError;
use rustyline::Editor;
// use signal_hook::iterator::Signals;
// use signal_hook::SIGINT;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use std::thread;

fn main() {
    handle_signals();

    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let commands: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

                // Check first for predefined commands
                if commands[0] == "cd" {
                    match cd(commands[1]) {
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
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => continue,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn handle_signals() {
    // match Signals::new(&[SIGINT]) {
    //     Ok(signals) => {
    //         thread::spawn(move || {
    //             for sig in signals.forever() {
    //                 println!("Received signal {:?}", sig);
    //             }
    //         });
    //     }
    //     Err(err) => println!("Could not init signal hooks, Error: {}", err),
    // };
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Could not init signal hooks.")
}

fn cd(path: &str) -> io::Result<()> {
    env::set_current_dir(Path::new(&path))
}
