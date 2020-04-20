use std::{fmt, io};

#[derive(Debug)]
pub struct ProcessError {
    pub kind: String,
    pub message: String,
}

impl From<io::Error> for ProcessError {
    fn from(error: io::Error) -> Self {
        ProcessError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}, msg: {}", self.kind, self.message)
    }
}
