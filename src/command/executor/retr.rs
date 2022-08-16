use std::path::Path;

use crate::{command::errors::ExecutionError, session::sessionstate::SessionState, status::Status};

use super::ExecutionResult;

pub(crate) fn retr_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let exists = Path::new(&argument).exists();

    match exists {
        false => Ok(ExecutionResult {
            status: 550,
            message: "File not found.".to_string(),
            new_state: None,
        }),
        true => Ok(ExecutionResult {
            status: 150,
            message: "Opening data connection.".to_string(),
            new_state: Some(SessionState {
                data_transfer_func: Some(data_transfer_func),
                ..state.clone()
            }),
        }),
    }
}

fn data_transfer_func(state: &SessionState, argument: &str) -> (Status, String) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_file_does_not_exist() {
        let state = SessionState::default();
        let result = retr_command_executor(&state, "/usr/jksdlfkjsd").unwrap();
        assert_eq!(result.status, 550);
        assert_eq!(result.message, "File not found.");
    }

    #[test]
    fn return_data_handler() {
        let state = SessionState::default();
        let result = retr_command_executor(&state, "/bin/sh").unwrap();
        assert_eq!(result.status, 150);
        assert_eq!(result.message, "Opening data connection.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert!(new_state.data_transfer_func.is_some());
        assert_eq!(
            new_state.data_transfer_func.unwrap() as usize,
            data_transfer_func as usize
        );
    }
}
