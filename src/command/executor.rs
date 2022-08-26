pub(super) mod acct;
pub(super) mod cdup;
pub(super) mod cwd;
pub(super) mod help;
pub(super) mod mode;
pub(super) mod noop;
pub(super) mod pass;
pub(super) mod pasv;
pub(super) mod port;
pub(super) mod pwd;
pub(super) mod rest;
pub(super) mod retr;
pub(super) mod stat;
pub(super) mod stru;
pub(super) mod syst;
pub(super) mod r#type;
pub(super) mod user;

use crate::{session::sessionstate::SessionState, status::Status};

use super::errors::ExecutionError;

pub(super) type Executor =
    fn(state: &SessionState, argument: &str) -> Result<ExecutionResult, ExecutionError>;

#[derive(Default)]
pub(crate) struct ExecutionResult {
    pub(crate) status: Status,
    pub(crate) message: String,
    pub(crate) new_state: Option<SessionState>,
}
