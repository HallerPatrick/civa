use std::env;
use std::path::Path;

use super::error::ProcessError;
use super::exit_status::ExitStatus;

#[allow(dead_code)]
pub fn cd(path: Option<&String>) -> Result<ExitStatus, ProcessError> {
    match path {
        Some(p) => set_cwd(p),
        None => set_cwd(&String::from(".")),
    }
}

fn set_cwd(path: &String) -> Result<ExitStatus, ProcessError> {
    match env::set_current_dir(Path::new(&path)) {
        Ok(()) => Ok(ExitStatus { code: 1 }),
        Err(_) => Err(ProcessError),
    }
}
