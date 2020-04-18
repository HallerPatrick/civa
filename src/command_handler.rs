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
//
//
//

use std::iter::Peekable;

const SEMICOLON: &str = ";";
const DOUBLE_AMPERSAND: &str = "&&";
const DOUBLE_PIPE: &str = "||";

#[derive(Debug)]
enum ExecStrategy {
    Builtin,
    SpecialBuiltin,
    Unspecific,
    ShellFunction,
    OtherUtilities,
    PathCommand,
}

#[derive(Debug)]
struct Command {
    command_name: String,
    arguments: Vec<String>,
    strategy: ExecStrategy,
}

// Returns single commands that are split by the special chars(sequences)
//
// Special delimiter:
//       1. ;
//       2. &&
//       3. ||
//
fn split_commands(command_string: &str) -> Vec<Vec<&str>> {
    let mut commands: Vec<Vec<&str>> = Vec::new();

    let mut command_tokens: Vec<&str> = command_string.split_whitespace().collect();

    while command_tokens.len() != 0 {
        if is_delimiter(command_tokens.first().unwrap()) {
            return commands;
        }

        let delimiter = has_next_delimiter_at(command_tokens.clone());

        match delimiter {
            Some(i) => {
                // TODO: MAKE THIS WORK
                let (left, right) = command_tokens.split_at(i);
                command_tokens = right.clone().to_vec();

                commands.push(left.clone().to_vec());
            }

            // No more delimiters -> one connected command
            None => {
                commands.push(command_tokens.clone());
                return commands;
                // // Empty
                // while !command_tokens.is_empty() {
                //     command_tokens.pop();
                // }
            }
        }
    }

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
    token.contains(SEMICOLON) || token.contains(DOUBLE_AMPERSAND) || token.contains(DOUBLE_PIPE)
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
}
