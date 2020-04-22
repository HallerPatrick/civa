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

use std::env::current_dir;

use super::editor::built_editor;
use super::editor::MyHelper;
use rustyline::Editor;

use termion::color::{Blue, Fg};
use termion::style::Reset;
use termion::terminal_size;

pub struct Cli {
    // configuration: Configuration
    pub editor: Editor<MyHelper>
}

impl Cli {
    pub fn new() -> Self {
        Self {
            editor: built_editor()
        }
    }

    fn get_cwd_label() -> String {
        format!(
            "{}{}{}",
            Fg(Blue),
            String::from(current_dir().unwrap().to_str().unwrap()),
            Reset
        )
    }

    pub fn update(&mut self) -> String {
        // let size: (u16, u16) = terminal_size().unwrap();
        // self.terminal_width = size.0;
        // self.terminal_height = size.1;

        let p = format!("{}> ", Cli::get_cwd_label());
        self.editor.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);

        p
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn name() {
        unimplemented!();
    }

}
