// Construct the command line cli
// In the future this should all be able to be customized through
// a config file.
//
// If I want to integrate python scripts as configs this could be
// a could first try.
// Source:  https://github.com/RustPython/RustPython
//
//
//
//
//
//
// Form own command line:
//
// {CWD} {SVN} {INDICATOR} ......................... {TIME || EXEC NO}
//
//
//

use log::debug;

use std::env::current_dir;

use termion::color::{Blue, Fg};
use termion::style::Reset;
use termion::terminal_size;

pub struct Cli {
    // configuration: Configuratio
    terminal_width: u16,
    terminal_height: u16,
}

impl Cli {
    pub fn new() -> Self {
        let size: (u16, u16) = terminal_size().unwrap();
        Self {
            terminal_width: size.0,
            terminal_height: size.1,
        }
    }
    pub fn get_prompt(&mut self) -> String {
        self.update();

        debug!(
            "Width: {}, Height: {}",
            self.terminal_width, self.terminal_height
        );

        // TODO: Find a way to print on the right site of terminal

        let mut line = Cli::get_cwd_label();

        line.push_str(" > ");
        // for _ in 2..self.terminal_width {
        //     line.push('.');
        // }

        line
    }

    fn get_cwd_label() -> String {
        format!(
            "{}{}{}",
            Fg(Blue),
            String::from(current_dir().unwrap().to_str().unwrap()),
            Reset
        )
    }

    fn update(&mut self) {
        let size: (u16, u16) = terminal_size().unwrap();
        self.terminal_width = size.0;
        self.terminal_height = size.1;
    }
}
