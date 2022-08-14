mod io;

use std::{
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
};

use log::{debug, error, info, warn};

use crate::{
    command::{self, errors::CommandError, verb::Verb, Command},
    config,
    session::io::write,
    status::Status,
};

use self::io::read_line;

struct Session {
    socket: TcpStream,
    read_socket: Box<dyn Read>,
    write_socket: Box<dyn Write>,
    state: SessionState,
}

#[derive(Clone)]
pub(crate) struct SessionState {
    pub(crate) user: Option<String>,
    pub(crate) is_authenticated: bool,
    pub(crate) previous_command: Option<Verb>,
    pub(crate) binary_flag: bool,
    pub(crate) name_prefix: PathBuf,

    has_greeted: bool,
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
        }
    }
}

#[derive(Debug, PartialEq)]
enum ShouldExit {
    No,
    Yes,
}

pub(crate) fn handle_new_connection(socket: TcpStream) {
    let mut session = Session::new(socket);
    run_session(&mut session, handle_pass);
}

impl Session {
    fn new(stream: TcpStream) -> Session {
        let read_socket = Box::new(stream.try_clone().expect("Failed to clone stream"));
        let write_socket = Box::new(stream.try_clone().expect("Failed to clone stream"));
        Session {
            socket: stream,
            read_socket,
            write_socket,
            state: SessionState::default(),
        }
    }
}

fn run_session(session: &mut Session, pass_handler: impl Fn(&mut Session) -> ShouldExit) {
    let peer_addr = session.socket.peer_addr().unwrap();
    debug!("New session started with peer {}", peer_addr);
    loop {
        match pass_handler(session) {
            ShouldExit::No => continue,
            ShouldExit::Yes => {
                end_session(&mut session.socket);
                break;
            }
        }
    }
    debug!("Session ended with peer {}", peer_addr);
}

fn handle_pass(session: &mut Session) -> ShouldExit {
    if !session.state.has_greeted {
        return handle_session_not_greeted(session);
    }

    let command = match await_command(&mut session.read_socket) {
        Ok(command) => command,
        Err((status, message)) => {
            return write_result_to_peer(&mut session.write_socket, status, &message)
        }
    };

    let ((status, message), result) = run_command(&command, &session.state);
    session.state = result;

    write_result_to_peer(&mut session.write_socket, status, &message)
}

fn handle_session_not_greeted(session: &mut Session) -> ShouldExit {
    session.state.has_greeted = true;
    greet_new_connection(&mut session.write_socket)
}

fn greet_new_connection(stream: &mut impl Write) -> ShouldExit {
    write_result_to_peer(stream, 220, "Welcome to the FeTP FTP server.")
}

fn write_result_to_peer(stream: &mut impl Write, status: Status, message: &str) -> ShouldExit {
    match write(stream, status, message) {
        Ok(written) => {
            info!("Wrote {} bytes.", written);
            ShouldExit::No
        }
        Err(error) => {
            error!("Failed to write response to client: {}", error);
            ShouldExit::Yes
        }
    }
}

fn await_command(stream: &mut impl Read) -> Result<Command, (Status, String)> {
    let command = parse_next_command(stream);

    match command {
        Ok(command) => Ok(command),
        Err(error) => {
            warn!("Error reading command: {}", error.to_string());
            return Err((500, error.to_string()));
        }
    }
}

fn parse_next_command(stream: &mut impl Read) -> Result<Command, CommandError> {
    match read_line(stream) {
        Ok(buffer) => command::parse(&buffer),
        Err(error) => {
            warn!("Error reading command: {}", error);
            Err(CommandError("Error reading command".to_string()))
        }
    }
}

// TODO handle errors
fn run_command(
    command: &Command,
    current_state: &SessionState,
) -> ((Status, String), SessionState) {
    let result = command.execute(current_state).unwrap();
    let mut new_state = result.new_state.unwrap_or(current_state.clone());
    new_state.previous_command = Some(command.verb.clone());
    ((result.status, result.message), new_state)
}

fn end_session(stream: &mut TcpStream) {
    stream
        .shutdown(std::net::Shutdown::Both)
        .expect("Failed to shutdown socket"); // This should never fail on linux
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

    #[derive(Default)]
    struct MockErrorStream {}
    impl Write for MockErrorStream {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            self.flush()?;
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Fake error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn run_session_test() {
        let test = |session: &mut Session| {
            match &session.state.user {
                None => session.state.user = Some("test".to_string()),
                Some(user) => {
                    if user == "test" {
                        session.state.user = Some("test2".to_string());
                    } else {
                        assert_eq!(user, "test2");
                        return ShouldExit::Yes;
                    }
                }
            }
            ShouldExit::No
        };

        let stream =
            TcpStream::connect("tcpbin.com:4242").expect("Unable to connect to tcpbin.com:4242");
        let mut session = Session::new(stream);

        run_session(&mut session, test);

        assert_eq!(session.state.user, Some("test2".to_string()));
        assert!(session.write_socket.write(b"buf").is_err());
    }

    #[test]
    fn handle_pass_greeting() {
        let stream =
            TcpStream::connect("tcpbin.com:4242").expect("Unable to connect to tcpbin.com:4242");
        let mut session = Session::new(stream);
        let result = handle_pass(&mut session);
        assert_eq!(result, ShouldExit::No);
        assert_eq!(session.state.has_greeted, true);
    }

    #[test]
    fn session_not_greeted_error() {
        let stream =
            TcpStream::connect("tcpbin.com:4242").expect("Unable to connect to tcpbin.com:4242");
        let mut session = Session::new(stream);
        session.write_socket = Box::new(MockErrorStream {});
        let result = handle_session_not_greeted(&mut session);
        assert_eq!(result, ShouldExit::Yes);
    }

    #[test]
    fn test_write_greeting() {
        let mut stream = MockStream::default();
        let result = greet_new_connection(&mut stream);
        assert_eq!(result, ShouldExit::No);
        assert_eq!(stream.out, b"220 Welcome to the FeTP FTP server.\r\n");
    }

    #[test]
    fn write_greeting_error() {
        let mut stream = MockErrorStream::default();
        let result = greet_new_connection(&mut stream);
        assert_eq!(result, ShouldExit::Yes);
    }

    #[test]
    fn await_command_correct() {
        let mut input = "USER foo\r\n".as_bytes();
        let res = await_command(&mut input);
        assert!(res.is_ok());
        let command = res.unwrap();
        assert_eq!(command.verb, Verb::USER);
        assert_eq!(command.arg, "foo");
    }

    #[test]
    fn await_command_err() {
        let mut input = "USR-foo\r\n".as_bytes();
        let res = await_command(&mut input);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().0, 500);
    }

    #[test]
    fn test_read_next_command_correct() {
        let mut input = "USER foo\r\n".as_bytes();
        let command = parse_next_command(&mut input);
        assert!(command.is_ok());
        let command = command.unwrap();
        assert_eq!(command.verb, Verb::USER);
        assert_eq!(command.arg, "foo");
    }

    #[test]
    fn test_read_next_command_incorrect() {
        let mut input = "USR-foo\r\n".as_bytes();
        let command = parse_next_command(&mut input);
        assert!(command.is_err());
    }

    #[test]
    fn test_read_more_than_one_command() {
        let mut input = "USER foo\r\nUSER bar\r\n".as_bytes();
        let command = parse_next_command(&mut input);
        assert!(command.is_ok());
        let command = command.unwrap();
        assert_eq!(command.verb, Verb::USER);
        assert_eq!(command.arg, "foo");
        let command = parse_next_command(&mut input);
        let command = command.unwrap();
        assert_eq!(command.verb, Verb::USER);
        assert_eq!(command.arg, "bar");
    }

    #[test]
    fn test_read_empty_command() {
        let mut input = "".as_bytes();
        let command = parse_next_command(&mut input);
        assert!(command.is_err());
    }

    #[test]
    fn test_read_io_error() {
        let mut input = ErrorStream {};
        let command = parse_next_command(&mut input);
        assert!(command.is_err());
    }

    #[test]
    fn test_handle_command() {
        let verb = Verb::USER;
        let command = Command {
            verb: Verb::USER,
            arg: "foo".to_string(),
        };
        let state = SessionState::default();
        let ((status, msg), new_state) = run_command(&command, &state);

        assert_eq!(new_state.user, Some("foo".to_string()));
        assert_eq!(new_state.is_authenticated, false);
        assert_eq!(new_state.previous_command, Some(verb));
        assert_eq!(status, 331);
        assert!(msg != "");
    }

    #[test]
    fn write_result_correct() {
        let mut stream = MockStream::default();
        let res = write_result_to_peer(&mut stream, 200, "foobar");
        assert_eq!(res, ShouldExit::No);
        assert_eq!(stream.out, b"200 foobar\r\n");
    }

    #[test]
    fn write_result_error() {
        let mut stream = MockErrorStream {};
        let res = write_result_to_peer(&mut stream, 200, "foobar");
        assert_eq!(res, ShouldExit::Yes);
    }

    #[test]
    fn test_end_session() {
        let mut stream =
            TcpStream::connect("tcpbin.com:4242").expect("Unable to connect to tcpbin.com:4242");
        let msg = b"test";
        assert!(stream.write(msg).is_ok());
        end_session(&mut stream);
        assert!(stream.write(msg).is_err());
    }

    #[test]
    fn test_end_session_already_closed() {
        let mut stream =
            TcpStream::connect("tcpbin.com:4242").expect("Unable to connect to tcpbin.com:4242");
        let msg = b"test";
        assert!(stream.write(msg).is_ok());
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        end_session(&mut stream);
        assert!(stream.write(msg).is_err());
    }
}
