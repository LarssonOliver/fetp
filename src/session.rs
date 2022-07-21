use std::{io::Read, net::TcpStream};

use log::{debug, error, info};

use crate::{
    command::{self, errors::CommandError, Command},
    connection::{self, Connection},
};

struct Session {
    connection: Connection,
    user: Option<String>,
    is_authenticated: bool,
}

pub fn new(socket: TcpStream) {
    let connection = Connection::new(socket);
    let mut session = Session::new(connection);
    session.run();
}

impl Session {
    fn new(connection: Connection) -> Session {
        Session {
            connection,
            user: None,
            is_authenticated: false,
        }
    }

    fn run(&mut self) {
        debug!("New session started");

        loop {
            let command = read_command(&mut self.connection);
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

fn read_command(connection: &mut Connection) -> Result<Command, CommandError> {
    let buffer = match connection.read() {
        Ok(buffer) => buffer,
        Err(err) => return Err(CommandError(err.to_string())),
    };

    command::parse(buffer.as_slice())
}
