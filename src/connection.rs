use std::{
    io::{Read, Write},
    net::TcpStream,
};

use log::info;

use crate::config;

pub struct Connection {
    socket: TcpStream,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection { socket }
    }

    pub fn write(&mut self, status: u16, message: &str) -> std::io::Result<usize> {
        log::debug!("Writing response: {} {}", status, message);
        write(&mut self.socket, status, message)
    }

    pub fn write_multiline(
        &mut self,
        status: u16,
        message_lines: &[&str],
    ) -> std::io::Result<Vec<usize>> {
        log::debug!("Writing multiline response: {} {:?}", status, message_lines);
        write_multiline(&mut self.socket, status, message_lines)
    }

    // TODO: Handle TELNET obligations.
    pub fn read(&mut self) -> std::io::Result<Vec<u8>> {
        read(&mut self.socket)
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        info!("Closing connection");
        self.socket.shutdown(std::net::Shutdown::Both)
    }

    pub fn write_then_close(&mut self, status: u16, message: &str) -> std::io::Result<usize> {
        let written = self.write(status, message)?;
        self.close()?;
        Ok(written)
    }
}

fn write(out: &mut dyn Write, status: u16, msg: &str) -> std::io::Result<usize> {
    let message_array = [msg];
    let result = write_multiline(out, status, message_array.as_slice())?;
    Ok(result[0])
}

fn write_multiline(out: &mut dyn Write, status: u16, msg: &[&str]) -> std::io::Result<Vec<usize>> {
    for line in msg {
        validate_outgoing_message(line)?;
    }

    let mut result: Vec<usize> = Vec::new();

    for (idx, line) in msg.iter().enumerate() {
        let out_str = if idx == msg.len() - 1 {
            format!("{} {}\r\n", status, line)
        } else {
            format!("{}-{}\r\n", status, line)
        };
        result.push(out.write(out_str.as_bytes())?);
    }

    Ok(result)
}

fn read(in_: &mut dyn Read) -> std::io::Result<Vec<u8>> {
    let mut buffer: [u8; config::MAX_LINE_LENGTH] = [0; config::MAX_LINE_LENGTH];
    let count = in_.read(&mut buffer)?;
    Ok(buffer[0..count].to_vec())
}

fn validate_outgoing_message(msg: &str) -> std::io::Result<()> {
    if msg.contains("\r") || msg.contains("\n") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Message contains newline characters",
        ));
    }

    Ok(())
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
    fn write_mutliline_correct() {
        let mut out = MockStream::new();
        write_multiline(&mut out, 220, &["foo", "bar"]).unwrap();
        assert_eq!(out.out, b"220-foo\r\n220 bar\r\n");
    }

    #[test]
    fn write_multiline_correct_single() {
        let mut out = MockStream::new();
        write_multiline(&mut out, 220, &["foo"]).unwrap();
        assert_eq!(out.out, b"220 foo\r\n");
    }

    #[test]
    fn write_mutliline_empty_msg_returns() {
        let mut out = MockStream::new();
        let res = write_multiline(&mut out, 220, &[]);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn write_error_if_message_contains_eol_chars() {
        let mut out = MockStream::new();
        let res = write(&mut out, 220, "Service ready\r");
        assert!(res.is_err());
        let res = write(&mut out, 220, "Service ready\n");
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
