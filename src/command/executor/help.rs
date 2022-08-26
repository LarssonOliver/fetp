use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn help_command_executor(
    _state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let result = match argument {
        "" => ExecutionResult {
            status: 211,
            message: concat!(
                "FeTP rust ftp server.\n",
                "https://github.com/larssonoliver/fetp"
            )
            .to_string(),
            new_state: None,
        },
        _ => ExecutionResult {
            status: 504,
            message: "Not implemented for this parameter.".to_string(),
            new_state: None,
        },
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_wihout_param() {
        let res = help_command_executor(&SessionState::default(), "").unwrap();
        assert_eq!(res.status, 211);
        assert_eq!(
            res.message,
            concat!(
                "FeTP rust ftp server.\n",
                "https://github.com/larssonoliver/fetp"
            )
        );
        assert!(res.new_state.is_none());
    }

    #[test]
    fn help_with_param() {
        let res = help_command_executor(&SessionState::default(), "foobar").unwrap();
        assert_eq!(res.status, 504);
        assert_eq!(res.message, "Not implemented for this parameter.",);
        assert!(res.new_state.is_none());
    }
}
