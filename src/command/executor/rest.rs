use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn rest_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let pos = match argument {
        "" => 0,
        _ => match usize::from_str_radix(argument, 10) {
            Ok(pos) => pos,
            Err(_) => return Ok(invalid_parameter()),
        },
    };

    let mut result = ExecutionResult {
        status: 350,
        message: format!("Start position set to {}.", pos),
        new_state: None,
    };

    if pos != state.file_offset {
        let mut new_state = state.clone();
        new_state.file_offset = pos;
        result.new_state = Some(new_state);
    }

    Ok(result)
}

fn invalid_parameter() -> ExecutionResult {
    ExecutionResult {
        status: 501,
        message: "Invalid parameter.".to_string(),
        new_state: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arg() {
        let mut state = SessionState::default();
        state.file_offset = 1337;
        let result = rest_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 350);
        assert_eq!(result.message, "Start position set to 0.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.file_offset, 0);
    }

    #[test]
    fn with_arg() {
        let mut state = SessionState::default();
        state.file_offset = 1337;
        let result = rest_command_executor(&state, "420").unwrap();
        assert_eq!(result.status, 350);
        assert_eq!(result.message, "Start position set to 420.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.file_offset, 420);
    }

    #[test]
    fn no_new_state_if_unchanged() {
        let mut state = SessionState::default();
        state.file_offset = 1337;
        let result = rest_command_executor(&state, "1337").unwrap();
        assert_eq!(result.status, 350);
        assert_eq!(result.message, "Start position set to 1337.");
        assert!(result.new_state.is_none());
    }

    #[test]
    fn invalid_args() {
        let state = SessionState::default();
        for param in ["foobar", "-213", "123.456"] {
            let result = rest_command_executor(&state, param).unwrap();
            assert_eq!(result.status, 501);
            assert_eq!(result.message, "Invalid parameter.");
            assert!(result.new_state.is_none());
        }
    }
}
