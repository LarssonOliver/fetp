use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn quit_command_executor(
    _state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    Ok(ExecutionResult {
        status: 221,
        message: "Bye.".to_string(),
        new_state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit() {
        let res = quit_command_executor(&SessionState::default(), "").unwrap();
        assert_eq!(res.status, 221);
        assert_eq!(res.message, "Bye.");
        assert!(res.new_state.is_none());
    }
}
