use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn noop_command_executor(
    _state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    Ok(ExecutionResult {
        status: 200,
        message: "NOOP".to_string(),
        new_state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop() {
        let state = SessionState::default();
        let res = noop_command_executor(&state, "").unwrap();
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "NOOP");
        assert!(res.new_state.is_none());
    }
}
