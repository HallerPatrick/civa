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

use super::editor::built_editor;
use super::editor::MyHelper;
use crate::config::command_bar::{
    command_bar_config_reader, CommandBarComponents, CommandBarConfig,
};
use crate::config::{ColorName, StyleName};

use git::GitCli;

use log::info;

use rustyline::Editor;
use termion::color as termion_colors;
use termion::color::Fg;
use termion::style;

pub struct Cli {
    pub configuration: CommandBarConfig,
    pub editor: Editor<MyHelper>,
}

impl Cli {
    pub fn new() -> Self {
        let conf = command_bar_config_reader("examples/.civa.bar.yaml").unwrap();

        info!("{:?}", conf);

        Self {
            editor: built_editor(),
            configuration: conf,
        }
    }

    fn get_cwd_label() -> String {
        format!(
            "{} ",
            String::from(current_dir().unwrap().to_str().unwrap()),
        )
    }

    pub fn update(&mut self) -> String {
        // let label = format!(
        //     "{}{}{}",
        //     Fg(Yellow),
        //     GitCli::get_current_branch().trim_end(),
        //     Reset
        // );

        // let p = format!("{} {}> ", Cli::get_cwd_label(), label);
        let p = self.build_cmd_bar();
        self.editor.helper_mut().expect("No helper").colored_prompt =
            format!("\x1b[1;32m{}\x1b[0m", p);

        p
    }

    fn get_current_user() -> String {
        var("USER").unwrap_or(String::new())
    }

    fn build_cmd_bar(&self) -> String {
        let mut vec: Vec<String> = Vec::with_capacity(self.configuration.components.len());
        for config in &self.configuration.components {
            // Paint Color
            match config.color.color_name {
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
                    vec.push(format!("{}", Fg(termion_colors::Blue)));
                }
                ColorName::GREEN => {
                    vec.push(format!("{}", Fg(termion_colors::Blue)));
                }
                _ => {
                    vec.push(format!("{}", Fg(termion_colors::Blue)));
                }
            };

            // Build Component Content
            match config.component_type {
                CommandBarComponents::CWD => vec.push(Cli::get_cwd_label()),
                CommandBarComponents::SVN => vec.push(format!(
                    "{} ",
                    GitCli::get_current_branch().trim_end().to_string()
                )),
                CommandBarComponents::PROMPT => vec.push(String::from("λ ")),
                CommandBarComponents::USER => vec.push(format!("{} ", Cli::get_current_user())),
                _ => {}
            }

            // Set Style
            match config.style.style_name {
                StyleName::BOLD => {
                    vec.push(format!("{}", style::Bold));
                }

                StyleName::ITALIC => {
                    vec.push(format!("{}", style::Italic));
                }
                _ => {}
            };

            vec.push(format!("{}", style::Reset));
        }

        info!("{:?}", vec);
        vec.join("")
    }
}
