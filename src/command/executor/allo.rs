use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn allo_command_executor(
    _state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    Ok(ExecutionResult {
        status: 502,
        message: "Command not implemented.".to_string(),
        new_state: None,
    })
}
