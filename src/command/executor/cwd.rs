use std::fs::canonicalize;

use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn cwd_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let mut path = state.name_prefix.clone();
    let decoded_argument = argument.replace('\0', "\n");
    path.push(decoded_argument);

    let result = match canonicalize(path) {
        Ok(realpath) => {
            let mut new_state = state.clone();
            new_state.name_prefix = realpath;
            ExecutionResult {
                status: 250,
                message: "Okay.".to_string(),
                new_state: Some(new_state),
            }
        }
        Err(error) => ExecutionResult {
            status: 550,
            message: error.to_string(),
            new_state: None,
        },
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn cange_dir_absolute() {
        let mut state = SessionState::default();
        let result = cwd_command_executor(&state, "/usr/bin").unwrap();
        assert_eq!(
            result.new_state.unwrap_or_default().name_prefix,
            PathBuf::from("/usr/bin")
        );
        assert_eq!(result.status, 250);
        assert_eq!(result.message, "Okay.");
        state.name_prefix.push("/usr/bin");
        let result = cwd_command_executor(&state, "/usr/lib").unwrap();
        assert_eq!(
            result.new_state.unwrap_or_default().name_prefix,
            PathBuf::from("/usr/lib")
        );
        assert_eq!(result.status, 250);
        assert_eq!(result.message, "Okay.");
    }

    #[test]
    fn change_dir() {
        let mut state = SessionState::default();
        state.name_prefix = PathBuf::from("/usr");
        let result = cwd_command_executor(&state, "bin").unwrap();
        assert_eq!(
            result.new_state.unwrap_or_default().name_prefix,
            PathBuf::from("/usr/bin")
        );
        assert_eq!(result.status, 250);
        assert_eq!(result.message, "Okay.");
        state.name_prefix = PathBuf::from("/usr/bin");
        let result = cwd_command_executor(&state, "../lib").unwrap();
        assert_eq!(
            result.new_state.unwrap_or_default().name_prefix,
            PathBuf::from("/usr/lib")
        );
        assert_eq!(result.status, 250);
        assert_eq!(result.message, "Okay.");
    }

    #[test]
    fn test_nonexisting_folder() {
        let state = SessionState::default();
        let result = cwd_command_executor(&state, "/lajsldf/lskdfj/djf").unwrap();
        assert_eq!(result.status, 550);
        assert_ne!(result.message, "");
        assert!(result.new_state.is_none());
    }
}
