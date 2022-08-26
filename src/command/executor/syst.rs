use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn syst_command_executor(
    _state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    Ok(ExecutionResult {
        status: 215,
        message: "UNIX Type: L8".to_string(),
        new_state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syst() {
        let state = SessionState::default();
        let res = syst_command_executor(&state, "").unwrap();
        assert_eq!(res.status, 215);
        assert_eq!(res.message, "UNIX Type: L8");
        assert!(res.new_state.is_none());
    }
}
