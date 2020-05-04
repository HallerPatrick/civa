mod builtins;
mod cli;
mod command;
mod config;
mod env;

use clap::{App, Arg};
use log::{info, LevelFilter};
use rustyline::error::ReadlineError;

use crate::cli::Cli;
use crate::command::executer::exec_sequentially;
use crate::command::handler::handle_commands;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate prettytable;

fn main() {
    let matches = App::new("civa")
        .version("0.1.0")
        .author("Patrick Haller <patrickhaller40@googlemail.com>")
        .about("A shell written in rust")
        .arg(
            Arg::with_name("loglevel")
                .short("l")
                .long("loglevel")
                .value_name("LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace"])
                .takes_value(true),
        )
        .get_matches();

    let loglevel = match matches.value_of("loglevel") {
        None => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => unreachable!(),
    };

    let mut builder = pretty_env_logger::formatted_builder();

    if let Ok(s) = std::env::var("RUST_LOG") {
        builder.parse_filters(&s);
    }

    builder.filter_module("nu", loglevel);
    builder.init();

    info!("Init Logger");

    // Start loop
    main_loop();
}

fn main_loop() {
    // let env_manager = EnvManager::new();
    info!("Init env manager");

    let mut cli = Cli::new();
    info!("Init Cli");

    loop {
        let p = cli.update();

        match cli.editor.readline(&p) {
            Ok(line) => {
                cli.editor.add_history_entry(line.as_str());
                info!("Read input line {}", line);
                let mut commands = handle_commands(line.as_str(), &cli.context);

                info!("Executing commands sequentially: {:?}", commands);
                exec_sequentially(&mut commands);
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
