use crate::{session::SessionState, status::Status};

use super::errors::CommandExecutionError;

pub(super) type Executor =
    fn(state: SessionState, argument: &str) -> Result<ExecutionResult, CommandExecutionError>;

#[derive(Default)]
pub(crate) struct ExecutionResult {
    pub(crate) status: Status,
    pub(crate) message: String,
    pub(crate) new_state: Option<SessionState>,
}

pub(super) fn user_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    let mut result = ExecutionResult::default();

    match argument {
        "" => {
            result.status = 501;
            result.message.push_str("User name parameter empty.")
        }
        "anonymous" => {
            result.status = 332;
            result.message.push_str("Need account for login.");
        }
        _ => {
            result.status = 331;
            result.message.push_str("User name okay, need password.");
        }
    }

    result.new_state = user_update_state(state, result.status, argument);
    Ok(result)
}

fn user_update_state(
    current_state: SessionState,
    status: Status,
    username: &str,
) -> Option<SessionState> {
    if status == 331 || status == 332 {
        let mut new_state = current_state;
        new_state.user = Some(username.to_string());
        new_state.is_authenticated = false;
        Some(new_state)
    } else {
        None
    }
}

pub(super) fn pass_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    unimplemented!()
}

pub(super) fn acct_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_valid_returns_331() {
        let state = SessionState::default();
        let argument = "foo";
        let result = user_command_executor(state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 331);
        assert_eq!(result.message, "User name okay, need password.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.user, Some(argument.to_string()));
        assert!(!new_state.is_authenticated);
    }

    #[test]
    fn user_anonymous_returns_332() {
        let state = SessionState::default();
        let argument = "anonymous";
        let result = user_command_executor(state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 332);
        assert_eq!(result.message, "Need account for login.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.user, Some(argument.to_string()));
        assert!(!new_state.is_authenticated);
    }

    #[test]
    fn user_no_argument_returns_501() {
        let state = SessionState::default();
        let argument = "";
        let result = user_command_executor(state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 501);
        assert_eq!(result.message, "User name parameter empty.");
        assert!(result.new_state.is_none());
    }
}
