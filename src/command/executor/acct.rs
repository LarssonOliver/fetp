use crate::{command::errors::CommandExecutionError, session::SessionState};

use super::ExecutionResult;

pub(crate) fn acct_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    unimplemented!()
}
