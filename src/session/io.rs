use std::io::{Read, Write};

use crate::status::Status;

pub fn write(out: &mut dyn Write, status: Status, msg: &str) -> std::io::Result<usize> {
    let message_array = [msg];
    let result = write_multiline(out, status, message_array.as_slice())?;
    Ok(result[0])
}

pub fn write_multiline(
    out: &mut dyn Write,
    status: Status,
    msg: &[&str],
) -> std::io::Result<Vec<usize>> {
    log::debug!("Writing response: {} {:?}", status, msg);

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

fn validate_outgoing_message(msg: &str) -> std::io::Result<()> {
    if msg.contains("\r") || msg.contains("\n") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Message contains newline characters",
        ));
    }

    Ok(())
}

// TODO: Handle TELNET obligations.
pub fn read_line(in_: &mut dyn Read) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();

    for byte in in_.bytes() {
        let char = match byte {
            Ok(char) => char,
            Err(error) => return Err(error),
        };

        buf.push(char);
        if char == b'\n' {
            break;
        }
    }
    Ok(buf)
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
            self.flush()?;
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
    fn read_line_into_vec() {
        let com = "220 Service ready\r\n";
        let mut mock = com.as_bytes();
        let buff = read_line(&mut mock).unwrap();
        assert_eq!(buff, com.as_bytes());
    }

    #[test]
    fn read_line_by_line() {
        let mut mock = "220 Service ready\r\n123 foo\n".as_bytes();
        assert_eq!(read_line(&mut mock).unwrap(), b"220 Service ready\r\n");
        assert_eq!(read_line(&mut mock).unwrap(), b"123 foo\n");
    }
}
