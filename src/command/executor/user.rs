use crate::{command::errors::ExecutionError, session::SessionState, status::Status};

use super::ExecutionResult;

pub(crate) fn user_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let mut result = ExecutionResult::default();

    match argument {
        "" => {
            result.status = 501;
            result.message.push_str("User name parameter empty.")
        }
        "anonymous" => {
            result.status = 230;
            result
                .message
                .push_str("Anonymous login ok, public access granted.");
        }
        _ => {
            result.status = 331;
            result.message.push_str("User name okay, need password.");
        }
    }

    result.new_state = user_update_state(&state, result.status, argument);
    Ok(result)
}

fn user_update_state(
    current_state: &SessionState,
    status: Status,
    username: &str,
) -> Option<SessionState> {
    if status == 230 || status == 331 {
        let mut new_state = current_state.clone();
        new_state.user = Some(username.to_string());
        new_state.is_authenticated = status == 230;
        Some(new_state)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::session::SessionState;

    use super::*;

    #[test]
    fn user_valid_returns_331() {
        let state = SessionState::default();
        let argument = "foo";
        let result = user_command_executor(&state, argument);
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
    fn user_anonymous_returns_230() {
        let state = SessionState::default();
        let argument = "anonymous";
        let result = user_command_executor(&state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 230);
        assert_eq!(result.message, "Anonymous login ok, public access granted.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.user, Some(argument.to_string()));
        assert!(new_state.is_authenticated);
    }

    #[test]
    fn user_no_argument_returns_501() {
        let state = SessionState::default();
        let argument = "";
        let result = user_command_executor(&state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 501);
        assert_eq!(result.message, "User name parameter empty.");
        assert!(result.new_state.is_none());
    }
}
