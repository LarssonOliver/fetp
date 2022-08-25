use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::PathBuf,
};

use crate::{command::verb::Verb, config, status::Status};

type TransferFunc = fn(
    parameter: &str,
    start_position: usize,
    read_stream: Option<&mut dyn Read>,
    write_stream: Option<&mut dyn Write>,
) -> (Status, String);

pub(crate) struct SessionState {
    pub(crate) user: Option<String>,
    pub(crate) is_authenticated: bool,
    pub(crate) previous_command: Option<Verb>,
    pub(crate) binary_flag: bool,
    pub(crate) name_prefix: PathBuf,
    pub(crate) has_greeted: bool,
    pub(crate) file_offset: usize,

    pub(crate) local_ip: Ipv4Addr,
    pub(crate) peer_ip: Ipv4Addr,

    pub(crate) port_ip: Option<SocketAddrV4>,
    pub(crate) data_listener: Option<TcpListener>,

    pub(crate) data_transfer_func: Option<TransferFunc>,
    pub(crate) data_transfer_func_parameter: Option<String>,

    pub(super) data_socket: Option<TcpStream>,
}

impl SessionState {
    pub fn new(local_ip: IpAddr, peer_ip: IpAddr) -> Self {
        let mut state = Self::default();
        state.local_ip = match local_ip {
            IpAddr::V4(ip) => ip,
            _ => panic!("Only IPv4 is supported"),
        };
        state.peer_ip = match peer_ip {
            IpAddr::V4(ip) => ip,
            _ => panic!("Only IPv4 is supported"),
        };
        state
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
            previous_command: None,
            binary_flag: false,
            has_greeted: false,
            name_prefix: PathBuf::from(config::NAME_PREFIX),
            data_listener: None,
            local_ip: Ipv4Addr::UNSPECIFIED,
            peer_ip: Ipv4Addr::UNSPECIFIED,
            port_ip: None,
            data_transfer_func: None,
            data_socket: None,
            file_offset: 0,
            data_transfer_func_parameter: None,
        }
    }
}

impl Clone for SessionState {
    // TODO This is called twice each command, should be once.
    fn clone(&self) -> Self {
        Self {
            user: self.user.clone(),
            is_authenticated: self.is_authenticated,
            previous_command: self.previous_command.clone(),
            binary_flag: self.binary_flag,
            has_greeted: self.has_greeted,
            name_prefix: self.name_prefix.clone(),
            data_listener: match self.data_listener {
                Some(ref listener) => Some(listener.try_clone().unwrap()),
                None => None,
            },
            local_ip: self.local_ip.clone(),
            peer_ip: self.peer_ip.clone(),
            port_ip: self.port_ip.clone(),
            data_transfer_func: self.data_transfer_func.clone(),
            data_socket: match self.data_socket {
                Some(ref socket) => Some(socket.try_clone().unwrap()),
                None => None,
            },
            file_offset: self.file_offset,
            data_transfer_func_parameter: self.data_transfer_func_parameter.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use super::*;

    #[test]
    #[should_panic]
    fn ipv6_should_panic() {
        let addr = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let ok = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let _ = SessionState::new(addr, ok);
    }

    #[test]
    #[should_panic]
    fn ipv6_peer_should_panic() {
        let addr = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let ok = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let _ = SessionState::new(ok, addr);
    }
}
