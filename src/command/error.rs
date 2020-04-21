use crate::builtins::error::BuiltinError;
use std::fmt;

#[derive(Debug)]
pub struct CommandError {
    pub kind: String,
    pub message: String,
}

impl From<BuiltinError> for CommandError {
    fn from(error: BuiltinError) -> Self {
        CommandError {
            kind: error.kind,
            message: error.message,
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}, msg: {}", self.kind, self.message)
    }
}
