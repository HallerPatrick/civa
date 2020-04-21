// pub mod builtin_names;
// pub mod cd;

pub mod builtin_names;
pub mod cd;
pub mod error;
pub mod executer;
pub mod exit_status;

pub static BUILTIN_NAMES: &'static [&str] = &["cd", ":q", "quit"];

// mod builtins {
//     pub mod builtin_names;
//     pub mod cd;
// }
