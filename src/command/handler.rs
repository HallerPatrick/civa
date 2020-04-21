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

use crate::command::{Command, ExecStrategy};
use crate::env::environment::EnvManager;
use log::{debug, info};

pub fn handle_commands(command_string: &str, env_manager: &EnvManager) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    let raw_commands: Vec<Vec<String>> = split_commands(command_string);

    for mut command in raw_commands {
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

    commands
}

fn define_command_strategy(command_name: &str, env_manager: &EnvManager) -> ExecStrategy {
    let builtin_names: Vec<&str> = vec!["cd", ":q"];

    // Check if command_name contains slash
    if has_slash(command_name) {
        // TODO: Handle slash command
        unimplemented!("Slash commands not implemented yet");
        // return ExecStrategy::Undefined;
    }

    // Check if command is a builtin utility
    if builtin_names.contains(&command_name) {
        ExecStrategy::Builtin

    // Check in PATH
    } else if env_manager.has_command(command_name) {
        ExecStrategy::PathCommand
    } else {
        ExecStrategy::Undefined
    }
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
