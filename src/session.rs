mod io;

use std::{io::Read, net::TcpStream};

use log::debug;

use crate::command::{self, errors::CommandError, Command};

struct Session {
    socket: TcpStream,
    user: Option<String>,
    is_authenticated: bool,
}

pub fn handle_new_connection(socket: TcpStream) {
    let mut session = Session::new(socket);
    session.run();
}

impl Session {
    fn new(socket: TcpStream) -> Session {
        Session {
            socket,
            user: None,
            is_authenticated: false,
        }
    }

    fn run(&mut self) {
        debug!("New session started");

        loop {
            // let command = read_command(&mut self.connection);
        }

        // match self
        //     .connection
        //     .write_then_close(421, "Service not implemented, closing connection.")
        // {
        //     Ok(len) => info!("Closed connection to peer after writing {} bytes", len),
        //     Err(err) => error!("Error writing to stream: {}", err),
        // }
    }

    fn execute(&mut self, command: Command) {}
}

// fn read_command(connection: &mut Connection) -> Result<Command, CommandError> {
//     let buffer = match connection.read() {
//         Ok(buffer) => buffer,
//         Err(err) => return Err(CommandError(err.to_string())),
//     };

//     command::parse(buffer.as_slice())
// }
