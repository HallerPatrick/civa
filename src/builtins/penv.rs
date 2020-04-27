use std::env;

use ansi_term::Colour::{Blue, Green};

use prettytable::Table;

// Pretty print the value of
// a environment variable
use crate::builtins::error::BuiltinError;
use crate::builtins::exit_status::ExitStatus;

pub fn penv(var_name: &str) -> Result<ExitStatus, BuiltinError> {
    if var_name == "" {
        return Err(BuiltinError {
            kind: String::from("penv"),
            message: String::from("No variable name provided"),
        });
    }

    let path_values = get_path_values(var_name);

    match path_values {
        Some(paths_string) => {
            let paths = paths_string.split(':');

            let mut table = Table::new();

            println!(
                "\n{}",
                Blue.paint(format!(
                    "ENVIRONMENT VARIABLE: {}",
                    Blue.bold().paint(var_name.to_uppercase())
                ))
            );

            for (i, path) in paths.enumerate() {
                table.add_row(row![Green.bold().paint((i + 1).to_string()), path]);
            }

            table.printstd();

            Ok(ExitStatus { code: 1 })
        }

        None => Err(BuiltinError {
            kind: String::from("penv"),
            message: String::from("Could not find environment variable"),
        }),
    }
}

fn get_path_values(var_name: &str) -> Option<String> {
    match env::var(var_name) {
        Ok(var) => Some(var),
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {

    use std::env;

    use super::*;

    #[test]
    fn test_get_path_values_success() {
        let key = "CIVA_ENV";
        let value = "SAMPLE_KEY";

        env::set_var(key, value);

        let result: Option<String> = get_path_values(&String::from(key));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), value);
    }

    // #[test]
    // fn test_get_path_values_unsuccessful() {
    //     let key = "CIVA_ENV";

    //     let result: Option<String> = get_path_values(&String::from(key));

    //     assert!(result.is_none());
    // }
}
