use crate::{command::errors::ExecutionError, session::SessionState};

use super::ExecutionResult;

pub(crate) fn rest_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
