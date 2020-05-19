use super::error::BuiltinError;
use crate::builtins::exit_status::ExitStatus;

use crate::config::ContextManager;

// TODO: Add $alias show
// TODO: Make this a binary??
pub fn alias(arguments: Vec<String>, ctx: &ContextManager) -> Result<ExitStatus, BuiltinError> {
    if arguments.len() != 2 {
        return Err(BuiltinError {
            kind: String::from("alias"),
            message: String::from(
                "No Space seperated alias name and alias value provied\n\t $ alias <key> <value>",
            ),
        });
    }

    let key: String = arguments[0].clone();
    let value: String = arguments[1].clone();

    if key == "" {
        return Err(BuiltinError {
            kind: String::from("alias"),
            message: String::from("No Alias Key provided"),
        });
    }

    match ctx.alias_system.borrow_mut().update_alias(key, value) {
        true => Ok(ExitStatus { code: 0 }),
        false => Ok(ExitStatus { code: 0 }),
    }
}
