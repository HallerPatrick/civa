mod builtins;
mod command_executer;
mod command_handler;
mod env;

use log::info;
use pretty_env_logger::init;

use command_executer::exec_sequentially;
use command_handler::handle_commands;

use env::environment::EnvManager;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    init();
    info!("Init Logger");

    let mut rl = Editor::<()>::new();
    info!("Init line reader");

    let env_manager = EnvManager::new();
    info!("Init env manager");

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                info!("Read input line {}", line);
                let commands = handle_commands(line.as_str(), &env_manager);

                info!("Executing commands sequentially: {:?}", commands);
                exec_sequentially(commands);
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
