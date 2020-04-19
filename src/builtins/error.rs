use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ProcessError;

impl Error for ProcessError {}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
