mod builtins;
mod cli;
mod command;
mod config;
mod env;

use clap::App;
use log::info;
use pretty_env_logger::init;
use rustyline::error::ReadlineError;

use crate::cli::Cli;
use crate::command::executer::exec_sequentially;
use crate::command::handler::handle_commands;
use crate::env::environment::EnvManager;

#[macro_use]
extern crate prettytable;

fn main() {
    App::new("civa")
        .version("0.1.0")
        .author("Patrick Haller <patrickhaller40@googlemail.com>")
        .about("A shell written in rust")
        .get_matches();

    init();

    info!("Init Logger");

    // Start loop
    main_loop();
}

fn main_loop() {
    let env_manager = EnvManager::new();
    info!("Init env manager");

    let mut cli = Cli::new();
    info!("Init Cli");

    loop {
        let p = cli.update();

        match cli.editor.readline(&p) {
            Ok(line) => {
                cli.editor.add_history_entry(line.as_str());
                info!("Read input line {}", line);
                let commands = handle_commands(line.as_str(), &env_manager);

                info!("Executing commands sequentially: {:?}", commands);
                exec_sequentially(commands);
            }
            // "Soft Reset" the shell

            // Ctrl-C
            Err(ReadlineError::Interrupted) => continue,

            // Ctrl-D
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
