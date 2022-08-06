mod io;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use log::{debug, error, warn};

use crate::{
    command::{self, errors::CommandError, verb::Verb, Command},
    session::io::write,
};

struct Session {
    socket: TcpStream,
    state: SessionState,
}

#[derive(Clone, Default)]
pub(crate) struct SessionState {
    pub(crate) user: Option<String>,
    pub(crate) is_authenticated: bool,
    pub(crate) previous_command: Option<Verb>,
}

pub(crate) fn handle_new_connection(socket: TcpStream) {
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
                previous_command: None,
            },
        }
    }

    fn run(&mut self) {
        debug!("New session started");

        if greet_new_connection(&mut self.socket).is_err() {
            error!("Failed to greet new connection, terminating session");
            return;
        }

        self.session_loop();

        self.end_session();

        debug!("Session ended");
    }

    fn session_loop(&mut self) {
        loop {
            let command = read_next_command(&mut self.socket);

            if command.is_err() {
                let error = command.as_ref().unwrap_err();
                warn!("Error reading command: {}", error.to_string());
                let written = write(&mut self.socket, 500, &error.to_string());
                if written.is_err() {
                    error!("Unable to write to socket, terminating session");
                    break;
                }
            }

            let command = command.unwrap();

            let result = execute_command(&command, &self.state);
            self.state = result;
        }
    }

    fn end_session(&mut self) {
        if let Err(error) = self.socket.shutdown(std::net::Shutdown::Both) {
            error!("Unable to shutdown socket: {}", error);
        };
    }
}

fn greet_new_connection(stream: &mut impl Write) -> Result<(), ()> {
    match write(stream, 220, "Welcome to the FeTP FTP server.") {
        Ok(_) => Ok(()),
        Err(error) => {
            error!("Unable to write to socket: {}", error);
            Err(())
        }
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

fn execute_command(command: &Command, state: &SessionState) -> SessionState {
    let verb = command.verb.clone();
    let result = command.execute(state.clone()).unwrap();
    let mut new_state = result.new_state.unwrap();
    new_state.previous_command = Some(verb);
    new_state
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

    #[derive(Default)]
    struct MockStream {
        out: Vec<u8>,
    }

    impl Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.out.extend_from_slice(buf);
            self.flush()?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_write_greeting() {
        let mut stream = MockStream::default();
        let result = greet_new_connection(&mut stream);
        assert!(result.is_ok());
        assert_eq!(stream.out, b"220 Welcome to the FeTP FTP server.\r\n");
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

    #[test]
    fn test_execute_command_with_state() {
        let verb = Verb::USER;
        let command = Command {
            verb: Verb::USER,
            arg: "foo".to_string(),
        };
        let state = SessionState::default();
        let new_state = execute_command(&command, &state);
        assert_eq!(new_state.user, Some("foo".to_string()));
        assert_eq!(new_state.is_authenticated, false);
        assert_eq!(new_state.previous_command, Some(verb));
    }
}
