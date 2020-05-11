mod builtins;
mod cli;
mod command;
mod config;
mod env;
mod status;

use crate::config::PyConfRuntime;
use clap::{App, Arg, SubCommand};
use log::{info, LevelFilter};
use pyo3::prelude::*;
use rustyline::error::ReadlineError;

use crate::cli::Cli;
use crate::command::executer::exec_sequentially;
use crate::command::handler::handle_commands;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate prettytable;

pub struct CivaOpts {
    pub pyconf_lib_path: String,
}

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
        .arg(Arg::with_name("civa-lib").short("cl").takes_value(true))
        // .arg(SubCommand::with_name("init"))
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

    let pyconf_lib_path = match matches.value_of("civa-lib") {
        Some(path) => String::from(path),
        None => String::new(),
    };

    let civa_opts = CivaOpts { pyconf_lib_path };

    // Start loop
    main_loop(civa_opts);
}

fn main_loop(civa_opts: CivaOpts) {
    info!("Init env manager");

    let mut cli = Cli::new(civa_opts);
    info!("Init Cli");

    let gil = Python::acquire_gil();
    let py_conf = PyConfRuntime::new(&gil);

    // TODO: IMPLS
    match py_conf.import_civa_lib() {
        false => info!("Could not find civa in site-packages"),
        true => info!("Found civa in site-packages"),
    }

    loop {
        let p = cli.update();

        match cli.editor.readline(&p) {
            Ok(line) => {
                cli.editor.add_history_entry(line.as_str());

                info!("Read input line {}", line);
                let mut commands = handle_commands(line.as_str(), &cli.context);

                info!("Executing commands sequentially: {:?}", commands);
                exec_sequentially(&mut commands, &cli.context);
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
