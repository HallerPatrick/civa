use std::fmt;

#[derive(Debug)]
pub struct EnvError {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}, msg: {}", self.kind, self.message)
    }
}
