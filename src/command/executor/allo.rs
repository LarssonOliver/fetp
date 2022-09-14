use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn allo_command_executor(
    _state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    Ok(ExecutionResult {
        status: 202,
        message: "Command not implemented, superfluous at this site.".to_string(),
        new_state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_accept() {
        let state = SessionState::default();
        for arg in ["", "foobar"] {
            let res = allo_command_executor(&state, arg).unwrap();
            assert_eq!(res.status, 202);
            assert_eq!(
                res.message,
                "Command not implemented, superfluous at this site."
            );
            assert!(res.new_state.is_none());
        }
    }
}
