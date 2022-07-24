mod io;

use std::{io::Read, net::TcpStream};

use log::debug;

use crate::command::{self, errors::CommandError, Command};

struct Session {
    socket: TcpStream,
    state: SessionState,
}

pub(crate) struct SessionState {
    pub(crate) user: Option<String>,
    pub(crate) is_authenticated: bool,
}

pub fn handle_new_connection(socket: TcpStream) {
    let mut session = Session::new(socket);
    session.run();
}

impl Session {
    fn new(socket: TcpStream) -> Session {
        Session {
            socket,
            state: SessionState {
                user: None,
                is_authenticated: false,
            },
        }
    }

    fn run(&mut self) {
        debug!("New session started");

        loop {
            let command = read_next_command(&mut self.socket);
        }

        // match self
        //     .connection
        //     .write_then_close(421, "Service not implemented, closing connection.")
        // {
        //     Ok(len) => info!("Closed connection to peer after writing {} bytes", len),
        //     Err(err) => error!("Error writing to stream: {}", err),
        // }
    }
}

fn read_next_command(stream: &mut impl Read) -> Result<Command, CommandError> {
    let buffer = match io::read(stream) {
        Ok(buffer) => buffer,
        Err(err) => return Err(CommandError(err.to_string())),
    };

    let buffer = trim_if_multiple_commands(&buffer);

    command::parse(buffer)
}

fn trim_if_multiple_commands(buffer: &[u8]) -> &[u8] {
    let last_index_of_first_crlf = buffer
        .iter()
        .position(|&b| b == b'\r')
        .unwrap_or(usize::MAX);

    match last_index_of_first_crlf {
        usize::MAX => buffer,
        _ => &buffer[..last_index_of_first_crlf],
    }
}

#[cfg(test)]
mod tests {
    use crate::command::verb::Verb;

    use super::*;

    struct ErrorStream {}
    impl Read for ErrorStream {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Fake error"))
        }
    }

    #[test]
    fn test_read_next_command_correct() {
        let mut input = "USER foo\r\n".as_bytes();
        let command = read_next_command(&mut input);
        assert!(command.is_ok());
        let command = command.unwrap();
        assert_eq!(command.verb, Verb::USER);
        assert_eq!(command.arg, "foo");
    }

    #[test]
    fn test_read_next_command_incorrect() {
        let mut input = "USER-foo\r\n".as_bytes();
        let command = read_next_command(&mut input);
        assert!(command.is_err());
    }

    #[test]
    fn test_read_more_than_one_command() {
        let mut input = "USER foo\r\nUSER bar\r\n".as_bytes();
        let command = read_next_command(&mut input);
        assert!(command.is_ok());
        let command = command.unwrap();
        assert_eq!(command.verb, Verb::USER);
        assert_eq!(command.arg, "foo");
        let command = read_next_command(&mut input);
        assert!(command.is_err());
    }

    #[test]
    fn test_read_empty_command() {
        let mut input = "".as_bytes();
        let command = read_next_command(&mut input);
        assert!(command.is_err());
    }

    #[test]
    fn test_read_io_error() {
        let mut input = ErrorStream {};
        let command = read_next_command(&mut input);
        assert!(command.is_err());
    }
}
