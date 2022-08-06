use crate::{command::errors::ExecutionError, session::SessionState};

use super::ExecutionResult;

pub(crate) fn acct_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    unimplemented!()
}
