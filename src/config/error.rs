use std::fmt;

#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config Error: {}", self.message)
    }
}
