use crate::{
    command::{errors::ExecutionError, verb::Verb},
    session::SessionState,
};

use super::ExecutionResult;

pub(crate) fn acct_command_executor(
    state: SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let mut result = ExecutionResult::default();

    if state.previous_command != Some(Verb::PASS) {
        result.status = 503;
        result.message.push_str("Previous command must be PASS.");
    } else if state.is_authenticated {
        result.status = 202;
        result.message.push_str("Access already granted.");
    } else {
        result.status = 530;
        result.message.push_str("Not logged in.");
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::command::verb::Verb;

    use super::*;

    #[test]
    fn acct_already_logged_in_202() {
        let mut state = SessionState::default();
        state.previous_command = Some(Verb::PASS);
        state.is_authenticated = true;

        let result = acct_command_executor(state, "").unwrap();

        assert_eq!(result.status, 202);
        assert_eq!(result.message, "Access already granted.")
    }

    #[test]
    fn acct_previous_command_not_pass_503() {
        let mut state = SessionState::default();
        state.previous_command = Some(Verb::USER);
        state.is_authenticated = true;

        let result = acct_command_executor(state, "").unwrap();

        assert_eq!(result.status, 503);
        assert_eq!(result.message, "Previous command must be PASS.")
    }

    #[test]
    fn acct_previous_login_rejected_530() {
        let mut state = SessionState::default();
        state.previous_command = Some(Verb::PASS);
        state.is_authenticated = false;

        let result = acct_command_executor(state, "").unwrap();

        assert_eq!(result.status, 530);
        assert_eq!(result.message, "Not logged in.")
    }
}
