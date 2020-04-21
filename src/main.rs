mod builtins;
mod cli;
mod command;
mod env;

use log::info;
use pretty_env_logger::init;

use cli::cli::Cli;
use cli::editor::built_editor;

use command::executer::exec_sequentially;
use command::handler::handle_commands;

use env::environment::EnvManager;

use rustyline::error::ReadlineError;

fn main() {
    init();
    info!("Init Logger");

    let env_manager = EnvManager::new();
    info!("Init env manager");

    let mut cli = Cli::new();
    info!("Init Cli");

    loop {
        // let prompt: String = cli.get_prompt();

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
