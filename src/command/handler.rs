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

use crate::command::PipeType::Undefined;
use log::{debug, info};

type CommandTokenCollection = Vec<Vec<String>>;

pub fn handle_commands(command_string: &str, env_manager: &EnvManager) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    // Only splits sequential commands, not pipes
    let raw_commands: CommandTokenCollection = split_commands(command_string);

    for mut command in raw_commands {
        // Pipes
        if command.clone().contains(&String::from("|")) {
            info!("Pipe is: {:?}", command);

            let mut pipe_commands = build_pipe_commands(command.clone(), env_manager);

            commands.append(pipe_commands.as_mut());
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
                pipe_type: Undefined,
            };

            commands.push(cmd);
        }
    }

    commands
}

fn build_pipe_commands(command: Vec<String>, env_manager: &EnvManager) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    let collected_commands = split_pipe(command);

    for (i, command) in collected_commands.iter().enumerate() {
        let current_pipe_type: PipeType;
        if commands.is_empty() {
            current_pipe_type = PipeType::PassesOutput;
        } else if collected_commands.len() == i + 1 {
            current_pipe_type = PipeType::ReceivesInput;
        } else {
            current_pipe_type = PipeType::OutAndInput;
        }

        if command.first().is_none() {
            continue;
        }

        // Owned it to closure as we dont need it afterwards -> speed?
        let mut command = command.to_owned();

        // Take command name
        let command_name: String = command.remove(0);

        let strategy = define_command_strategy(command_name.as_str(), env_manager);

        commands.push(Command {
            command_name,
            // Rest of command -> arguments
            arguments: command,
            strategy,
            pipe_type: current_pipe_type,
        });
    }
    commands
}

fn split_pipe(raw_pipe_commands: Vec<String>) -> CommandTokenCollection {
    let mut commands: CommandTokenCollection = Vec::new();

    let mut current_command = Vec::new();

    for command in raw_pipe_commands {
        if command == "|" {
            commands.push(current_command);
            current_command = Vec::new();
        } else {
            current_command.push(command);
        }
    }

    commands.push(current_command);

    commands
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

fn has_slash(token: &str) -> bool {
    token.contains('/')
}

// Returns single commands that are split by the special chars(sequences)
//
// Special delimiter:
//       1. ;
//       2. &&
//       3. ||
//
fn split_commands(command_string: &str) -> CommandTokenCollection {
    let mut commands: CommandTokenCollection = Vec::new();

    let mut command_tokens: Vec<&str> = command_string.split_whitespace().collect();

    while !command_tokens.is_empty() {
        if is_delimiter(command_tokens.first().unwrap()) && command_tokens.len() == 1 {
            return commands;
        }

        let delimiter = has_next_delimiter_at(&command_tokens);

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
                        .map(|s| (*s).to_string())
                        .collect(),
                );
                return commands;
            }
        }
    }

    debug!("Split input into: {:?}", commands);
    commands
}

fn has_next_delimiter_at(tokens: &[&str]) -> Option<usize> {
    for (i, token) in tokens.iter().enumerate() {
        if is_delimiter(token) {
            return Some(i);
        }
    }
    None
}

fn is_delimiter(token: &str) -> bool {
    token.contains(';') || token.contains("&&") || token.contains("||")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pipe() {
        let pip_commands = vec![
            String::from("a"),
            String::from("-la"),
            String::from("|"),
            String::from("b"),
        ];

        let result = vec![
            vec![String::from("a"), String::from("-la")],
            vec![String::from("b")],
        ];
        assert_eq!(split_pipe(pip_commands), result);
    }

    #[test]
    fn test_split_pipe_2() {
        let pip_commands = vec![String::from("echo"), String::from("|"), String::from("exa")];

        let result = vec![vec![String::from("echo")], vec![String::from("exa")]];
        assert_eq!(split_pipe(pip_commands), result);
    }

    #[test]
    fn test_build_pipe_commands() {
        let cmd: Vec<String> = vec![
            String::from("ls"),
            String::from("|"),
            String::from("echo"),
            String::from("Hello"),
        ];

        let env_mananger = EnvManager::new();

        let expected_result = vec![
            Command {
                command_name: String::from("ls"),
                arguments: Vec::<String>::new(),
                pipe_type: PipeType::PassesOutput,
                strategy: ExecStrategy::PathCommand,
            },
            Command {
                command_name: String::from("echo"),
                arguments: vec![String::from("Hello")],
                pipe_type: PipeType::ReceivesInput,
                strategy: ExecStrategy::PathCommand,
            },
        ];

        assert_eq!(expected_result, build_pipe_commands(cmd, &env_mananger));
    }

    #[test]
    fn test_build_pipe_commands_3_pipes() {
        let cmd: Vec<String> = vec![
            String::from("ls"),
            String::from("-la"),
            String::from("|"),
            String::from("echo"),
            String::from("|"),
            String::from("exa"),
        ];

        let env_manager = EnvManager::new();
        let expected_result = vec![
            Command {
                command_name: String::from("ls"),
                arguments: vec![String::from("-la")],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::PassesOutput,
            },
            Command {
                command_name: String::from("echo"),
                arguments: Vec::<String>::new(),
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::OutAndInput,
            },
            Command {
                command_name: String::from("exa"),
                arguments: Vec::<String>::new(),
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::ReceivesInput,
            },
        ];

        assert_eq!(expected_result, build_pipe_commands(cmd, &env_manager));
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

        let result = has_next_delimiter_at(&v);

        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_has_next_delimiter_at_found_none() {
        let v = vec!["ls", "some"];

        let result = has_next_delimiter_at(&v);

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

    #[test]
    fn test_handle_commands() {
        let command_string = "cd .. || ls || echo | ls";
        let env_manager = EnvManager::new();

        let commands: Vec<Command> = handle_commands(command_string, &env_manager);

        let expected_result = vec![
            Command {
                command_name: String::from("cd"),
                arguments: vec![String::from("..")],
                strategy: ExecStrategy::Builtin,
                pipe_type: PipeType::Undefined,
            },
            Command {
                command_name: String::from("/bin/ls"),
                arguments: vec![],
                strategy: ExecStrategy::PathCommand,
                pipe_type: Undefined,
            },
            Command {
                command_name: String::from("echo"),
                arguments: vec![],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::PassesOutput,
            },
            Command {
                command_name: String::from("ls"),
                arguments: vec![],
                strategy: ExecStrategy::PathCommand,
                pipe_type: PipeType::ReceivesInput,
            },
        ];

        assert_eq!(expected_result, commands);
    }
}
