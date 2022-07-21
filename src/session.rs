use std::net::TcpStream;

use log::{error, info};

use crate::{command::Command, connection::Connection};

struct Session {
    connection: Connection,
}

pub fn new(socket: TcpStream) {
    let connection = Connection::new(socket);
    let mut session = Session::new(connection);
    session.run();
}

impl Session {
    fn new(connection: Connection) -> Session {
        Session { connection }
    }

    fn run(&mut self) {
        match self
            .connection
            .write_then_close(421, "Service not implemented, closing connection.")
        {
            Ok(len) => info!("Closed connection to peer after writing {} bytes", len),
            Err(err) => error!("Error writing to stream: {}", err),
        }
    }

    fn execute(&mut self, command: Command) {}
}
