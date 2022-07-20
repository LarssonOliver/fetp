use crate::connection::Connection;

pub struct Session {
    connection: Connection,
}

impl Session {
    pub fn new(connection: Connection) -> Session {
        Session { connection }
    }
}
