// POSIX Standard Handling of Commands
// link: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_09_01_01
//
// Process/Rules:
//
// Rules 1: If the command does not contain any slashes
//     a) Then look for builtin utility
//     b) If the command is of the list than the result is unspecified?? (see link)
//     c) If command is a function know to the shell, execute it
//     (https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_09_05)
//     d) If command is of another list of utilities (type and ulimit) invoke those (owm impl?)
//     e) Otherwise search in PATH (more specifications in link)
//
// Rules 2: If command contains at least 1 shlash, the shell shall execute in a seperate utility
//          environment.
//
//
// Example:
//
//
// 1) cd .. -> Rule 1, builtin utility
//
//
//

// The job of the command handler is to receive the raw input string from the
// command line it the parses the string to extract all tokens and commands
// correctly.
//
// For every command a Command object is contructed and passed to the
// command executer
//

use crate::builtins::BUILTIN_NAMES;
use crate::command::{Command, ExecStrategy, PipeType};
use crate::env::environment::EnvManager;

use log::{debug, info};

pub fn handle_commands(command_string: &str, env_manager: &EnvManager) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    // Only splits sequentiall commands, not pipes
    let raw_commands: Vec<Vec<String>> = split_commands(command_string);

    info!("Splitted commands: {:?}", raw_commands);

    for mut command in raw_commands {
        // Pipes
        if command.clone().contains(&String::from("|")) {
            info!("Found pipe");
            let mut pipe_commands = build_pipe_commands(command.clone());
            info!("Pipe list: {:?}", pipe_commands);
            commands.append(&mut pipe_commands);
        } else {
            let mut command_name = command.remove(0);

            let strategy = define_command_strategy(command_name.as_str(), env_manager);

            info!("Defined strategy: {:?}", strategy);

            match strategy {
                ExecStrategy::Builtin => {
                    // Do nothing?
                }
                ExecStrategy::PathCommand => {
                    command_name = env_manager.get_expanded(command_name).unwrap().into()
                }
                _ => {}
            }

            let cmd = Command {
                command_name,
                arguments: command,
                strategy,
            };

            commands.push(cmd);
        }
    }

    commands
}

fn build_pipe_commands(mut command: Vec<String>) -> Vec<Command> {
    let mut pipe_arguments: Vec<Command> = Vec::new();

    let mut current_comand_type: PipeType = PipeType::OnlyInput;
    let mut current_command_tokens: Vec<String> = Vec::new();

    while !command.is_empty() {
        info!("Current commands left: {:?}", command);
        info!("Current command token: {:?}", current_command_tokens);

        // Build new command
        if command.first().unwrap() == "|" || command.len() == 1 {
            let command_name: String = current_command_tokens.remove(0);

            pipe_arguments.push(Command {
                command_name,
                arguments: current_command_tokens.clone(),
                strategy: ExecStrategy::Pipe(current_comand_type.clone()),
            });

            current_command_tokens = Vec::new();
            command.remove(0);
        }

        while command.len() > 0 {
            if command.first().unwrap() != "|" {
                current_command_tokens.push(command.remove(0))
            } else {
                break;
            }

            if command.len() == 0 {
                let command_name: String = current_command_tokens.remove(0);

                pipe_arguments.push(Command {
                    command_name,
                    arguments: current_command_tokens.clone(),
                    strategy: ExecStrategy::Pipe(PipeType::OnlyOutput),
                });
            }
        }

        if command.len() == 1 {
            current_comand_type = PipeType::OnlyOutput
        } else {
            current_comand_type = PipeType::OutAndInput
        }
    }

    pipe_arguments
}

fn define_command_strategy(command_name: &str, env_manager: &EnvManager) -> ExecStrategy {
    // Check if command_name contains slash
    if has_slash(command_name) {
        // TODO: Handle slash command
        unimplemented!("Slash commands not implemented yet");
        // return ExecStrategy::Undefined;
    }

    // Check if command is a builtin utility
    if BUILTIN_NAMES.contains(&command_name) {
        ExecStrategy::Builtin

    // Check in PATH
    } else if env_manager.has_command(command_name) {
        ExecStrategy::PathCommand
    } else {
        ExecStrategy::Undefined
    }
}

fn is_pipe(token: &str) -> bool {
    token.contains("|") && !token.contains("||")
}

fn has_slash(token: &str) -> bool {
    token.contains("/")
}

// Returns single commands that are split by the special chars(sequences)
//
// Special delimiter:
//       1. ;
//       2. &&
//       3. ||
//
fn split_commands(command_string: &str) -> Vec<Vec<String>> {
    let mut commands: Vec<Vec<String>> = Vec::new();

    let mut command_tokens: Vec<&str> = command_string.split_whitespace().collect();

    while !command_tokens.is_empty() {
        if is_delimiter(command_tokens.first().unwrap()) && command_tokens.len() == 1 {
            return commands;
        }

        let delimiter = has_next_delimiter_at(command_tokens.clone());

        match delimiter {
            // Take all command tokens till delimiter
            Some(i) => {
                let mut single_command: Vec<String> = Vec::new();

                for _ in 0..(i) {
                    let token = command_tokens.remove(0);
                    single_command.push(String::from(token));
                }

                // Remove delimiter
                command_tokens.remove(0);

                commands.push(single_command);
            }

            // No more delimiters -> one connected command
            None => {
                commands.push(
                    command_tokens
                        .clone()
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                );
                return commands;
            }
        }
    }

    debug!("Split input into: {:?}", commands);
    commands
}

fn has_next_delimiter_at(tokens: Vec<&str>) -> Option<usize> {
    let mut i: usize = 0;
    for token in tokens {
        if is_delimiter(token) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn is_delimiter(token: &str) -> bool {
    token.contains(";") || token.contains("&&") || token.contains("||")
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_is_pipe() {
        assert_eq!(is_pipe("ls"), false);
    }

    #[test]
    fn test_is_pipe_sequential_command() {
        assert_eq!(is_pipe("ls || ls"), false);
    }

    #[test]
    fn test_is_pipe_pipe() {
        assert_eq!(is_pipe("ls | ls"), true);
    }

    #[test]
    fn test_build_pipe_commands() {
        let cmd: Vec<String> = vec![String::from("ls"), String::from("|"), String::from("echo")];

        let expected_result = vec![
            Command {
                command_name: String::from("ls"),
                arguments: Vec::<String>::new(),
                strategy: ExecStrategy::Pipe(PipeType::OnlyOutput),
            },
            Command {
                command_name: String::from("echo"),
                arguments: Vec::<String>::new(),
                strategy: ExecStrategy::Pipe(PipeType::OnlyInput),
            },
        ];

        assert_eq!(expected_result, build_pipe_commands(cmd));
    }

    #[test]
    fn test_test() {
        assert_eq!(is_delimiter("ls"), false);
    }

    #[test]
    fn test_is_delimiter_false() {
        assert_eq!(is_delimiter("ls"), false);
    }

    #[test]
    fn test_is_delimiter_true() {
        assert_eq!(is_delimiter("||"), true);
    }

    #[test]
    fn test_has_next_delimiter_at_found() {
        let v = vec!["ls", "||", "some"];

        let result = has_next_delimiter_at(v);

        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_has_next_delimiter_at_found_none() {
        let v = vec!["ls", "some"];

        let result = has_next_delimiter_at(v);

        assert_eq!(result, None);
    }

    #[test]
    fn test_split_commands_empty() {
        let c = "";

        let result = split_commands(c);

        let expected_result: Vec<Vec<&str>> = vec![];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_commands_single_command_name() {
        let c = "ls";

        let result = split_commands(c);

        let expected_result: Vec<Vec<&str>> = vec![vec!["ls"]];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_commands_single_delimiter() {
        let c = ";";

        let result = split_commands(c);

        let expected_result: Vec<Vec<&str>> = vec![];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_commands_single_command_with_args() {
        let c = "ls -la";

        let result = split_commands(c);

        let expected_result: Vec<Vec<&str>> = vec![vec!["ls", "-la"]];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_commands_commands() {
        let c = "ls -la || cd ..";

        let result = split_commands(c);

        let expected_result: Vec<Vec<&str>> = vec![vec!["ls", "-la"], vec!["cd", ".."]];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_commands_commands_delimiter_at_end() {
        let c = "ls -la || cd .. &&";

        let result = split_commands(c);

        let expected_result: Vec<Vec<&str>> = vec![vec!["ls", "-la"], vec!["cd", ".."]];
        assert_eq!(result, expected_result);
    }
}
