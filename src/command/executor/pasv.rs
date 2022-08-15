use std::net::TcpListener;

use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

pub(crate) fn pasv_command_executor(
    state: &SessionState,
    _argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let ip = state.local_ip.octets();
    let port = listener.local_addr().unwrap().port();

    let mut new_state = state.clone();
    new_state.data_listener = Some(listener);

    let message = format!(
        "={},{},{},{},{},{}",
        ip[0],
        ip[1],
        ip[2],
        ip[3],
        port / 256,
        port % 256
    );

    Ok(ExecutionResult {
        status: 227,
        message,
        new_state: Some(new_state),
    })
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::*;

    #[test]
    fn test_passive_mode() {
        let state = SessionState::default();
        let result = pasv_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 227);
        assert!(result.message.starts_with('='));

        let addr: Vec<&str> = result.message[1..].split(",").collect();
        assert_eq!(addr.len(), 6);
        for part in addr {
            assert!(part.parse::<u8>().is_ok());
        }
        assert!(result.new_state.is_some());
        assert!(result.new_state.unwrap().data_listener.is_some());
    }

    #[test]
    fn passive_mode_already_listening() {
        let mut state = SessionState::default();
        state.data_listener = Some(TcpListener::bind("127.0.0.1:29743").unwrap());
        let result = pasv_command_executor(&state, "").unwrap();
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert!(new_state.data_listener.is_some());
        assert_ne!(
            new_state
                .data_listener
                .unwrap()
                .local_addr()
                .unwrap()
                .port(),
            29743
        );
    }
}
