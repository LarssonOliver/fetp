use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::config;

pub struct Connection {
    socket: TcpStream,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection { socket }
    }

    pub fn write(&mut self, status: u16, message: &str) -> std::io::Result<usize> {
        write(&mut self.socket, status, message)
    }

    pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
        read(&mut self.socket)
    }
}

fn write(out: &mut dyn Write, status: u16, msg: &str) -> std::io::Result<usize> {
    if msg.contains("\r") || msg.contains("\n") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Message contains newline characters",
        ));
    }

    let out_str = format!("{} {}\r\n", status, msg);
    out.write(out_str.as_bytes())
}

fn read(in_: &mut dyn Read) -> std::io::Result<Vec<u8>> {
    let mut buffer: [u8; config::MAX_LINE_LENGTH] = [0; config::MAX_LINE_LENGTH];
    let count = in_.read(&mut buffer)?;
    Ok(buffer[0..count].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStream {
        out: Vec<u8>,
    }

    impl MockStream {
        fn new() -> MockStream {
            MockStream { out: Vec::new() }
        }
    }

    impl Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.out.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_writes_correct_to_out() {
        let mut out = MockStream::new();
        write(&mut out, 220, "Service ready").unwrap();
        assert_eq!(out.out, b"220 Service ready\r\n");
    }

    #[test]
    fn write_error_if_message_contains_eol_chars() {
        let mut out = MockStream::new();
        let res = write(&mut out, 220, "Service ready\r\n");
        assert!(res.is_err());
    }

    #[test]
    fn read_into_vec() {
        let com = "220 Service ready\r\n";
        let mut mock = com.as_bytes();
        let buff = read(&mut mock).unwrap();
        assert_eq!(buff, com.as_bytes());
    }
}
