pub mod alias;
pub mod builtin_names;
pub mod cd;
pub mod error;
pub mod executer;
pub mod exit_status;
pub mod penv;

pub static BUILTIN_NAMES: &[&str] = &["cd", ":q", "quit", "penv"];
