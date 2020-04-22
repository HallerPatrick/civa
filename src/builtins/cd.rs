use std::env;
use std::path::Path;

use super::error::BuiltinError;
use super::exit_status::ExitStatus;

pub fn cd(path: Option<&String>) -> Result<ExitStatus, BuiltinError> {
    match path {
        Some(p) => set_cwd(p),
        None => set_cwd(&String::from(".")),
    }
}

fn set_cwd(path: &str) -> Result<ExitStatus, BuiltinError> {
    match env::set_current_dir(Path::new(path)) {
        Ok(()) => Ok(ExitStatus { code: 1 }),
        Err(_) => Err(BuiltinError {
            kind: String::from("Builtin:cd"),
            message: String::from("Could not execute cd command"),
        }),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_cd_to_current_dir() {
        let result = cd(Some(&String::from(".")));

        // Should always pass
        assert!(result.is_ok());
    }

    #[test]
    fn test_cd_to_current_dir_without_provided_path() {
        let result = cd(None);

        // Should always pass
        assert!(result.is_ok());
    }



    #[test]
    fn test_cd_to_not_existing_dir() {
        let result = cd(Some(&String::from("not_existing_dir")));

        // Should always pass
        assert!(result.is_err());
    }

    #[test]
    fn test_set_cwd_to_current_dir() {
        let result = cd(Some(&String::from(".")));

        // Should always pass
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_cwd_to_current_dir_not_existing_dir() {
        let result = cd(Some(&String::from("Not existing dir")));

        // Should always pass
        assert!(result.is_err());
    }
}
