use std::{fmt, io};

#[derive(Debug)]
pub struct BuiltinError {
    pub kind: String,
    pub message: String,
}

impl From<io::Error> for BuiltinError {
    fn from(error: io::Error) -> Self {
        BuiltinError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl fmt::Display for BuiltinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}, msg: {}", self.kind, self.message)
    }
}
