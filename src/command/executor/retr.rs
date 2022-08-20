use std::io::{Read, Write};

use log::{info, warn};

use crate::{command::errors::ExecutionError, session::sessionstate::SessionState, status::Status};

use super::ExecutionResult;

pub(crate) fn retr_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let file = state.name_prefix.join(argument);
    let exists = file.exists();

    match exists {
        false => Ok(ExecutionResult {
            status: 550,
            message: "File not found.".to_string(),
            new_state: None,
        }),
        true => {
            let mut new_state = state.clone();
            new_state.data_transfer_func = Some(data_transfer_func);
            new_state.data_transfer_func_parameter = Some(file.to_str().unwrap().to_string());
            Ok(ExecutionResult {
                status: 150,
                message: "Opening data connection.".to_string(),
                new_state: Some(new_state),
            })
        }
    }
}

// TODO handle MODE (bin, ascii)
fn data_transfer_func(
    argument: &str,
    start_position: usize,
    _read_stream: Option<&mut dyn Read>,
    write_stream: Option<&mut dyn Write>,
) -> (Status, String) {
    let stream = match write_stream {
        Some(stream) => stream,
        None => return (425, "No data connection was established.".to_string()),
    };

    let file = match std::fs::read(argument) {
        Ok(file) => file,
        Err(error) => {
            warn!("Error reading file: {}", error);
            return (551, "Server error.".to_string());
        }
    };

    match stream.write(&file[start_position..]) {
        Ok(count) => {
            info!("File sent, {} bytes transmitted.", count);
            (226, "Transfer complete.".to_string())
        }
        Err(_) => (426, "Error while sending data.".to_string()),
    }
}

#[cfg(test)]
mod tests {

    use std::{io::BufWriter, path::PathBuf, vec};

    use super::*;

    #[test]
    fn error_file_does_not_exist() {
        let state = SessionState::default();
        let result = retr_command_executor(&state, "/usr/jksdlfkjsd").unwrap();
        assert_eq!(result.status, 550);
        assert_eq!(result.message, "File not found.");
    }

    #[test]
    fn relative_file_exists() {
        let mut state = SessionState::default();
        state.name_prefix = PathBuf::from("/bin");
        let result = retr_command_executor(&state, "sh").unwrap();
        assert_eq!(result.status, 150);
        assert_eq!(result.message, "Opening data connection.");
    }

    #[test]
    fn return_data_handler() {
        let state = SessionState::default();
        let result = retr_command_executor(&state, "/bin/sh").unwrap();
        assert_eq!(result.status, 150);
        assert_eq!(result.message, "Opening data connection.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert!(new_state.data_transfer_func.is_some());
        assert_eq!(
            new_state.data_transfer_func.unwrap() as usize,
            data_transfer_func as usize
        );
    }

    #[test]
    fn handle_no_connection() {
        let (status, msg) = data_transfer_func("", 0, Some(&mut "".as_bytes()), None);

        assert_eq!(status, 425);
        assert_eq!(msg, "No data connection was established.");
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
    fn handle_write_error() {
        let (status, msg) =
            data_transfer_func("/bin/sh", 0, None, Some(&mut MockErrorStream::default()));
        assert_eq!(status, 426);
        assert_eq!(msg, "Error while sending data.");
    }

    #[test]
    fn handle_disk_error() {
        let (status, msg) = data_transfer_func("", 0, None, Some(&mut BufWriter::new(vec![])));
        assert_eq!(status, 551);
        assert_eq!(msg, "Server error.");
    }

    #[test]
    fn write_file_to_out() {
        let mut writer = BufWriter::new(vec![]);
        let (status, msg) = data_transfer_func("/bin/sh", 0, None, Some(&mut writer));
        assert_eq!(status, 226);
        assert_eq!(msg, "Transfer complete.");
        assert_ne!(writer.into_inner().unwrap().len(), 0);
    }

    #[test]
    fn start_position() {
        let mut writer = BufWriter::new(vec![]);
        let _ = data_transfer_func("/bin/sh", 0, None, Some(&mut writer));
        let writer_contents = writer.into_inner().unwrap();
        let mut offset_writer = BufWriter::new(vec![]);
        let _ = data_transfer_func("/bin/sh", 100, None, Some(&mut offset_writer));
        let offset_writer_contents = offset_writer.into_inner().unwrap();

        assert_eq!(writer_contents.len(), offset_writer_contents.len() + 100);
        let equal_count = writer_contents[100..]
            .iter()
            .zip(&offset_writer_contents)
            .filter(|&(a, b)| a == b)
            .count();

        assert_eq!(equal_count, writer_contents.len() - 100);
    }
}
