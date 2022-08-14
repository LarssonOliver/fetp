use crate::{command::errors::ExecutionError, session::SessionState};

use super::ExecutionResult;

pub(crate) fn cdup_command_executor(
    state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let mut new_state = state.clone();
    new_state.name_prefix.pop();

    Ok(ExecutionResult {
        message: "Okay.".to_string(),
        status: 200,
        new_state: Some(new_state),
    })
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn change_to_parent_dir() {
        let mut state = SessionState::default();
        state.name_prefix = PathBuf::from("/usr/bin");
        let result = cdup_command_executor(&state, "").unwrap();
        assert!(result.new_state.is_some());
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Okay.");
        assert_eq!(result.new_state.unwrap().name_prefix, Path::new("/usr"));

        state.name_prefix = PathBuf::from("/usr");
        let result = cdup_command_executor(&state, "").unwrap();
        assert!(result.new_state.is_some());
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Okay.");
        assert_eq!(result.new_state.unwrap().name_prefix, Path::new("/"));
    }

    #[test]
    fn change_when_root() {
        let mut state = SessionState::default();
        state.name_prefix = PathBuf::from("/");
        let result = cdup_command_executor(&state, "").unwrap();
        assert!(result.new_state.is_some());
        assert_eq!(result.status, 200);
        assert_eq!(result.message, "Okay.");
        assert_eq!(result.new_state.unwrap().name_prefix, Path::new("/"));
    }
}
