use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn stru_command_executor(
    _state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let (status, message) = match argument {
        "F" | "f" => (200, "Using file structure."),
        "" => (504, "Parameter required."),
        _ => (504, "Only file structure is supported."),
    };

    Ok(ExecutionResult {
        status,
        message: message.to_string(),
        new_state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_file() {
        let state = SessionState::default();
        for arg in ["F", "f"] {
            let result = stru_command_executor(&state, arg).unwrap();
            assert_eq!(result.status, 200);
            assert_eq!(result.message, "Using file structure.")
        }
    }

    #[test]
    fn reject_other_arg() {
        let state = SessionState::default();
        let result = stru_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 504);
        assert_eq!(result.message, "Parameter required.");
        let result = stru_command_executor(&state, "foobar").unwrap();
        assert_eq!(result.status, 504);
        assert_eq!(result.message, "Only file structure is supported.");
    }
}
