pub(super) mod acct;
pub(super) mod pass;
pub(super) mod user;

use crate::{session::SessionState, status::Status};

use super::errors::CommandExecutionError;

pub(super) type Executor =
    fn(state: SessionState, argument: &str) -> Result<ExecutionResult, CommandExecutionError>;

#[derive(Default)]
pub(crate) struct ExecutionResult {
    pub(crate) status: Status,
    pub(crate) message: String,
    pub(crate) new_state: Option<SessionState>,
}
