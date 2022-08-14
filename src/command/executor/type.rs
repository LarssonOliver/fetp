use crate::{command::errors::ExecutionError, session::SessionState};

use super::ExecutionResult;

pub(crate) fn type_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let mut new_state = state.clone();

    let (status, message) = match argument.to_ascii_uppercase().as_str() {
        "I" | "L 8" => {
            new_state.binary_flag = true;
            (200, "Binary mode enabled.")
        }
        "A" | "A N" => {
            new_state.binary_flag = false;
            (200, "Binary mode disabled.")
        }
        "" => (504, "Parameter required."),
        _ => (504, "Invalid parameter."),
    };

    Ok(ExecutionResult {
        message: message.to_string(),
        status,
        new_state: Some(new_state),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_flag_false_by_default() {
        let state = SessionState::default();
        assert_eq!(state.binary_flag, false);
    }

    #[test]
    fn turn_binary_on() {
        let state = SessionState::default();
        let result = type_command_executor(&state, "I").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Binary mode enabled.");
        assert_eq!(result.new_state.unwrap().binary_flag, true);

        let result = type_command_executor(&state, "L 8").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Binary mode enabled.");
        assert_eq!(result.new_state.unwrap().binary_flag, true);
    }

    #[test]
    fn turn_binary_off() {
        let mut state = SessionState::default();
        state.binary_flag = true;
        let result = type_command_executor(&state, "A").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Binary mode disabled.");
        assert_eq!(result.new_state.unwrap().binary_flag, false);

        let result = type_command_executor(&state, "A N").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Binary mode disabled.");
        assert_eq!(result.new_state.unwrap().binary_flag, false);
    }

    #[test]
    fn ivalid_parameter_returns_504() {
        let state = SessionState::default();
        let result = type_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 504);
        assert_eq!(result.message, "Parameter required.");
        let result = type_command_executor(&state, "foo").unwrap();
        assert_eq!(result.status, 504);
        assert_eq!(result.message, "Invalid parameter.");
    }

    #[test]
    fn support_any_case_param() {
        let mut state = SessionState::default();
        let result = type_command_executor(&state, "i").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.new_state.unwrap().binary_flag, true);
        let result = type_command_executor(&state, "l 8").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.new_state.unwrap().binary_flag, true);

        state.binary_flag = false;
        let result = type_command_executor(&state, "a").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.new_state.unwrap().binary_flag, false);
        let result = type_command_executor(&state, "a n").unwrap();
        assert_eq!(result.status, 200);
        assert_eq!(result.new_state.unwrap().binary_flag, false);
    }
}
