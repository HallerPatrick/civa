pub mod editor;

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

use std::env::{current_dir, var};

use crate::config::command_bar::CommandBarComponents;
use crate::config::ContextManager;

use crate::config::{ColorName, StyleName};
use editor::built_editor;
use editor::MyHelper;

use git::GitCli;

use rustyline::Editor;
use termion::color as termion_colors;
use termion::color::Fg;
use termion::style;

pub struct Cli {
    pub context: ContextManager,
    pub editor: Editor<MyHelper>,
}

impl Cli {
    pub fn new() -> Self {
        // let conf = command_bar_config_reader("examples/.civa.bar.yaml").unwrap();

        let context = ContextManager::init();

        // info!("{:?}", conf);

        Self {
            editor: built_editor(),
            context,
        }
    }

    fn get_cwd_label() -> String {
        format!("{}", String::from(current_dir().unwrap().to_str().unwrap()),)
    }

    pub fn update(&mut self) -> String {
        let p = self.build_cmd_bar();
        self.editor.helper_mut().expect("No helper").colored_prompt =
            format!("\x1b[1;32m{}\x1b[0m", p);

        p
    }

    fn get_current_user() -> String {
        var("USER").unwrap_or_default()
    }

    fn push_color(vec: &mut Vec<String>, color_name: &ColorName) {
        // Paint Color
        match color_name {
            ColorName::YELLOW => {
                vec.push(format!("{}", Fg(termion_colors::Yellow)));
            }
            ColorName::WHITE => {
                vec.push(format!("{}", Fg(termion_colors::White)));
            }

            ColorName::BLUE => {
                vec.push(format!("{}", Fg(termion_colors::Blue)));
            }

            ColorName::RED => {
                vec.push(format!("{}", Fg(termion_colors::Red)));
            }
            ColorName::GREEN => {
                vec.push(format!("{}", Fg(termion_colors::Green)));
            }
            _ => {
                vec.push(format!("{}", Fg(termion_colors::Blue)));
            }
        };
    }

    fn push_component_content(vec: &mut Vec<String>, component_type: &CommandBarComponents) {
        match component_type {
            CommandBarComponents::CWD => vec.push(Cli::get_cwd_label()),
            CommandBarComponents::SVN => vec.push(format!(
                "{}",
                GitCli::get_current_branch().trim_end().to_string()
            )),
            CommandBarComponents::USER => vec.push(format!("{}", Cli::get_current_user())),
            _ => {}
        }
    }

    fn push_style(vec: &mut Vec<String>, style_name: &StyleName) {
        match style_name {
            StyleName::BOLD => {
                vec.push(format!("{}", style::Bold));
            }

            StyleName::ITALIC => {
                vec.push(format!("{}", style::Italic));
            }
            _ => {}
        };
    }

    fn build_cmd_bar(&self) -> String {
        let mut vec: Vec<String> =
            Vec::with_capacity(self.context.command_bar_config.components.len());
        for config in &self.context.command_bar_config.components {
            Cli::push_color(&mut vec, &config.color.color_name);

            vec.push(config.sorround.left.clone());

            // Build Component Content
            Cli::push_component_content(&mut vec, &config.component_type);

            vec.push(config.sorround.right.clone());
            vec.push(String::from(" "));

            // Set Style
            Cli::push_style(&mut vec, &config.style.style_name);

            vec.push(format!("{}", style::Reset));
        }

        // Add prompt
        Cli::push_color(
            &mut vec,
            &self.context.command_bar_config.prompt.color.color_name,
        );
        vec.push(self.context.command_bar_config.prompt.sorround.left.clone());
        vec.push(self.context.command_bar_config.prompt.symbol.clone());
        vec.push(
            self.context
                .command_bar_config
                .prompt
                .sorround
                .right
                .clone(),
        );
        vec.push(String::from(" "));
        Cli::push_style(
            &mut vec,
            &self.context.command_bar_config.prompt.style.style_name,
        );
        vec.push(format!("{}", style::Reset));

        // info!("{:?}", vec);
        vec.join("")
    }
}

#[cfg(test)]
mod tests_push_color {

    use super::*;

    #[test]
    fn push_color_blue() {
        let mut vec: Vec<String> = Vec::new();

        Cli::push_color(&mut vec, &ColorName::BLUE);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.first().unwrap(), "\u{1b}[38;5;4m");
    }

    #[test]
    fn push_color_yellow() {
        let mut vec: Vec<String> = Vec::new();

        Cli::push_color(&mut vec, &ColorName::YELLOW);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.first().unwrap(), "\u{1b}[38;5;3m");
    }

    #[test]
    fn push_color_red() {
        let mut vec: Vec<String> = Vec::new();

        Cli::push_color(&mut vec, &ColorName::RED);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.first().unwrap(), "\u{1b}[38;5;1m");
    }

    #[test]
    fn push_color_green() {
        let mut vec: Vec<String> = Vec::new();

        Cli::push_color(&mut vec, &ColorName::GREEN);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.first().unwrap(), "\u{1b}[38;5;2m");
    }
}

#[cfg(test)]
mod tests_push_style {

    use super::*;

    #[test]
    fn push_style() {
        let mut vec: Vec<String> = Vec::new();

        Cli::push_style(&mut vec, &StyleName::NORMAL);
        Cli::push_style(&mut vec, &StyleName::BOLD);
        Cli::push_style(&mut vec, &StyleName::ITALIC);

        assert_eq!(vec!["\u{1b}[1m", "\u{1b}[3m"], vec);
    }
}
