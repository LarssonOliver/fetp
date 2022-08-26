use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn stat_command_executor(
    _state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    Ok(ExecutionResult {
        status: 502,
        message: "Not implemented.".to_string(),
        new_state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_implemented() {
        let res = stat_command_executor(&SessionState::default(), "").unwrap();
        assert_eq!(res.status, 502);
        assert_eq!(res.message, "Not implemented.");
        assert!(res.new_state.is_none());
    }
}
