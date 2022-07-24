use crate::{session::SessionState, status::Status};

use super::errors::CommandExecutionError;

pub(super) type Executor =
    fn(state: SessionState, argument: &str) -> Result<ExecutionResult, CommandExecutionError>;

pub struct ExecutionResult {
    status: Status,
    message: String,
    new_state: Option<SessionState>,
}

pub(super) fn user_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    unimplemented!()
}

pub(super) fn pass_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    unimplemented!()
}

pub(super) fn acct_command_executor(
    state: SessionState,
    argument: &str,
) -> Result<ExecutionResult, CommandExecutionError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
