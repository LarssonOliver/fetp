use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn mode_command_executor(
    _state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let (status, message) = match argument {
        "S" | "s" => (200, "Using stream mode."),
        "" => (504, "Parameter required."),
        _ => (504, "Only stream mode is supported."),
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
        for arg in ["S", "s"] {
            let result = mode_command_executor(&state, arg).unwrap();
            assert_eq!(result.status, 200);
            assert_eq!(result.message, "Using stream mode.")
        }
    }

    #[test]
    fn reject_other_arg() {
        let state = SessionState::default();
        let result = mode_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 504);
        assert_eq!(result.message, "Parameter required.");
        let result = mode_command_executor(&state, "foobar").unwrap();
        assert_eq!(result.status, 504);
        assert_eq!(result.message, "Only stream mode is supported.");
    }
}
