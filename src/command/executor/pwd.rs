use crate::{command::errors::ExecutionError, session::SessionState};

use super::ExecutionResult;

pub(crate) fn pwd_command_executor(
    state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let path = state.name_prefix.to_str().unwrap();
    let path_enocded = path.replace('\n', "\0");
    let message = format!("\"{}\"", path_enocded);

    Ok(ExecutionResult {
        status: 257,
        new_state: None,
        message,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config;

    use super::*;

    #[test]
    fn print_name_prefix() {
        let mut state = SessionState::default();
        let result = pwd_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 257);
        assert_eq!(result.message, format!("\"{}\"", config::NAME_PREFIX));
        state.name_prefix = PathBuf::from("/foo/bar");
        let result = pwd_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 257);
        assert_eq!(result.message, "\"/foo/bar\"");
    }

    #[test]
    fn test_lf_encode() {
        let mut state = SessionState::default();
        state.name_prefix = PathBuf::from("/foo\n/bar");
        let result = pwd_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 257);
        assert_eq!(result.message, "\"/foo\0/bar\"");
    }
}
