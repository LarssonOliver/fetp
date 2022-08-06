use crate::{
    auth,
    command::{errors::ExecutionError, verb::Verb},
    session::SessionState,
};

use super::ExecutionResult;

pub(crate) fn pass_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    pass_command_executor_with_validator(state, argument, auth::validate)
}

fn pass_command_executor_with_validator(
    state: SessionState,
    argument: &str,
    validator: fn(&str, &str) -> bool,
) -> Result<ExecutionResult, ExecutionError> {
    let mut result = ExecutionResult::default();

    if state.previous_command != Some(Verb::USER) {
        result.status = 503;
        result.message.push_str("Previous command must be USER.")
    } else if is_anonymous_user(&state) {
        result.status = 202;
        result.message.push_str("Already logged in as anonymous.");
    } else if let "" = argument {
        result.status = 501;
        result.message.push_str("Password parameter empty.")
    } else if validator(&state.user.as_ref().unwrap(), argument) {
        result.status = 230;
        result.message.push_str("User logged in, proceed.");

        let mut new_state = state;
        new_state.is_authenticated = true;
        result.new_state = Some(new_state);
    } else {
        result.status = 530;
        result.message.push_str("User name or password incorrect.");
    }

    Ok(result)
}

fn is_anonymous_user(state: &SessionState) -> bool {
    state.is_authenticated && state.user == Some("anonymous".to_string())
}

#[cfg(test)]
mod tests {
    use crate::command::verb::Verb;

    use super::*;

    #[test]
    fn pass_valid_credentials_returns_230() {
        let validator = |u: &str, p: &str| u == "foo" && p == "bar";
        let mut state = SessionState::default();
        state.user = Some("foo".to_string());
        state.previous_command = Some(Verb::USER);
        let argument = "bar";
        let result = pass_command_executor_with_validator(state, argument, validator);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 230);
        assert_eq!(result.message, "User logged in, proceed.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.user, Some("foo".to_string()));
        assert!(new_state.is_authenticated);
    }

    #[test]
    fn pass_no_argument_returns_501() {
        let mut state = SessionState::default();
        state.user = Some("foo".to_string());
        state.previous_command = Some(Verb::USER);
        let argument = "";
        let result = pass_command_executor(state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 501);
        assert_eq!(result.message, "Password parameter empty.");
        assert!(result.new_state.is_none());
    }

    #[test]
    fn pass_anonymous_202() {
        let mut state = SessionState::default();
        state.user = Some("anonymous".to_string());
        state.previous_command = Some(Verb::USER);
        state.is_authenticated = true;
        let argument = "";
        let result = pass_command_executor(state, argument);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 202);
        assert_eq!(result.message, "Already logged in as anonymous.");
        assert!(result.new_state.is_none());
    }

    #[test]
    fn pass_invalid_credentials_530() {
        let validator = |u: &str, p: &str| u == "foo" && p == "bar";
        let mut state = SessionState::default();
        state.user = Some("foo".to_string());
        state.previous_command = Some(Verb::USER);
        let argument = "baz";
        let result = pass_command_executor_with_validator(state, argument, validator);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 530);
        assert_eq!(result.message, "User name or password incorrect.");
        assert!(result.new_state.is_none());
    }

    #[test]
    fn pass_last_command_must_be_user_503() {
        let validator = |u: &str, p: &str| u == "foo" && p == "bar";
        let mut state = SessionState::default();
        state.user = Some("foo".to_string());
        state.previous_command = Some(Verb::ACCT);
        let argument = "bar";
        let result = pass_command_executor_with_validator(state, argument, validator);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 503);
        assert_eq!(result.message, "Previous command must be USER.");
        assert!(result.new_state.is_none());
    }
}
