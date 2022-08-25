use std::net::{Ipv4Addr, SocketAddrV4};

use crate::{command::errors::ExecutionError, session::sessionstate::SessionState};

use super::ExecutionResult;

const BYTES_IN_IP_WITH_PORT: usize = 6;

pub(crate) fn port_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let bytes: Vec<u8> = argument
        .split(",")
        .map(|x| x.parse::<u8>())
        .take_while(Result::is_ok)
        .map(Result::unwrap)
        .collect();

    if bytes.len() != BYTES_IN_IP_WITH_PORT {
        return Ok(invalid_format());
    }

    let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
    let port = ((bytes[4] as u16) << 8) | bytes[5] as u16;

    let port_ip = SocketAddrV4::new(ip, port);
    let mut new_state = state.clone();
    new_state.port_ip = Some(port_ip);

    // If a pasv listener is active, it should be shut down when a port
    // request comes in.
    new_state.data_listener = None;

    Ok(ExecutionResult {
        status: 200,
        message: "Okay.".to_string(),
        new_state: Some(new_state),
    })
}

fn invalid_format() -> ExecutionResult {
    ExecutionResult {
        status: 501,
        message: "Invalid argument.".to_string(),
        new_state: None,
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, TcpListener};

    use super::*;

    #[test]
    fn parse_port() {
        let arg = "127,0,0,1,221,103";
        let state = SessionState::default();
        let res = port_command_executor(&state, &arg).unwrap();
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "Okay.");
        assert!(res.new_state.is_some());
        let new_state = res.new_state.unwrap();
        assert!(new_state.port_ip.is_some());
        let portip = new_state.port_ip.unwrap();
        assert_eq!(portip.ip(), &Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(portip.port(), 221 * 256 + 103);
    }

    #[test]
    fn invalid_format() {
        let state = SessionState::default();
        for arg in [
            "123,12,13,2,,",
            "foobar",
            "1,2,3,4,5,6,7,8,9",
            "-12,0,0,1,200,100",
            "256,0,0,1,200,100",
        ] {
            let res = port_command_executor(&state, &arg).unwrap();
            assert_eq!(res.status, 501);
            assert_eq!(res.message, "Invalid argument.");
            assert!(res.new_state.is_none());
        }
    }

    #[test]
    fn shutsdown_passive_listener() {
        let mut state = SessionState::default();
        state.data_listener = Some(TcpListener::bind("0.0.0.0:0").unwrap());
        let res = port_command_executor(&state, "127,0,0,1,100,200").unwrap();
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "Okay.");
        assert!(res.new_state.is_some());
        let new_state = res.new_state.unwrap();
        assert!(new_state.data_listener.is_none());
        assert!(new_state.port_ip.is_some());
    }
}
