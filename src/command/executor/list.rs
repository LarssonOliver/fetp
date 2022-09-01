use std::io::{Read, Write};

use crate::{command::errors::ExecutionError, session::sessionstate::SessionState, status::Status};

use super::ExecutionResult;

pub(crate) fn list_command_executor(
    state: &SessionState,
    argument: &str,
) -> Result<ExecutionResult, ExecutionError> {
    let mut path = state.name_prefix.to_owned();

    if !argument.is_empty() {
        path = path.join(argument);
    }

    let exists = path.exists();

    match exists {
        false => Ok(ExecutionResult {
            status: 550,
            message: "File not found.".to_string(),
            new_state: None,
        }),
        true => {
            let mut new_state = state.clone();
            new_state.data_transfer_func = Some(data_transfer_func);
            new_state.data_transfer_func_parameter = Some(path.to_str().unwrap().to_string());
            Ok(ExecutionResult {
                status: 150,
                message: "Opening data connection.".to_string(),
                new_state: Some(new_state),
            })
        }
    }
}

fn data_transfer_func(
    argument: &str,
    start_position: usize,
    _read_stream: Option<&mut dyn Read>,
    write_stream: Option<&mut dyn Write>,
) -> (Status, String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{fs, io::BufWriter};

    use super::*;

    #[test]
    fn current_dir_no_param_returns_data_handler() {
        let mut state = SessionState::default();
        state.name_prefix.push("/usr");
        let result = list_command_executor(&state, "").unwrap();
        assert_eq!(result.status, 150);
        assert_eq!(result.message, "Opening data connection.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert!(new_state.data_transfer_func.is_some());
        assert_eq!(
            new_state.data_transfer_func.unwrap() as usize,
            data_transfer_func as usize
        );
        assert!(new_state.data_transfer_func_parameter.is_some());
        assert_eq!(new_state.data_transfer_func_parameter.unwrap(), "/usr");
    }

    #[test]
    fn absolute_path() {
        let state = SessionState::default();
        let result = list_command_executor(&state, "/usr").unwrap();
        assert_eq!(result.status, 150);
        assert_eq!(result.message, "Opening data connection.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.data_transfer_func_parameter.unwrap(), "/usr");
    }

    #[test]
    fn relative_path() {
        let result = list_command_executor(&SessionState::default(), "usr/bin").unwrap();
        assert_eq!(result.status, 150);
        assert_eq!(result.message, "Opening data connection.");
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.data_transfer_func_parameter.unwrap(), "/usr/bin");
    }

    #[test]
    fn path_does_not_exist() {
        let result = list_command_executor(&SessionState::default(), "alolashdf").unwrap();
        assert_eq!(result.status, 550);
        assert_eq!(result.message, "File not found.");
        assert!(result.new_state.is_none());
    }

    #[test]
    fn handle_no_connection() {
        let (status, msg) = data_transfer_func("", 0, None, None);
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
    fn handle_disk_permission_error() {
        let (status, msg) = data_transfer_func("/root", 0, None, Some(&mut BufWriter::new(vec![])));
        assert_eq!(status, 451);
        assert_eq!(msg, "Error reading directory or file.");
    }

    #[test]
    fn write_directory_contents() {
        let mut writer = BufWriter::new(vec![]);
        let (status, msg) = data_transfer_func("/bin", 0, None, Some(&mut writer));
        assert_eq!(status, 226);
        assert_eq!(msg, "Transfer complete.");
        let outbuf = writer.into_inner().unwrap();
        let outstr = String::from_utf8(outbuf).unwrap();
        let lines: Vec<&str> = outstr.split("\r\n").collect();

        let dir = fs::read_dir("/bin").unwrap();
        let contents: Vec<String> = dir
            .take_while(Result::is_ok)
            .map(|x| x.unwrap().file_name().into_string())
            .take_while(Result::is_ok)
            .map(Result::unwrap)
            .collect();

        assert_eq!(contents.len(), lines.len() - 1);

        for item in contents {
            assert!(lines.iter().any(|x| x.ends_with(&item)));
        }
    }

    #[test]
    fn write_argument_is_regular_file() {
        let mut writer = BufWriter::new(vec![]);
        let (status, msg) = data_transfer_func("/bin/sh", 0, None, Some(&mut writer));
        assert_eq!(status, 226);
        assert_eq!(msg, "Transfer complete.");
        let outbuf = writer.into_inner().unwrap();
        assert_eq!(String::from_utf8(outbuf).unwrap(), "sh\r\n");
    }

    #[test]
    fn directory_types() {
        let mut writer = BufWriter::new(vec![]);
        let (status, msg) = data_transfer_func("/", 0, None, Some(&mut writer));
        assert_eq!(status, 226);
        assert_eq!(msg, "Transfer complete.");

        let outbuf = writer.into_inner().unwrap();
        let outstr = String::from_utf8(outbuf).unwrap();
        let lines: Vec<&str> = outstr.split("\r\n").collect();

        for line in lines {
            if line.ends_with(" usr") {
                assert!(line.starts_with("drwxr-xr-x 1 owner group"));
            }
        }
    }

    #[test]
    fn file_types() {
        let mut writer = BufWriter::new(vec![]);
        let (status, msg) = data_transfer_func("/bin/sh", 0, None, Some(&mut writer));
        assert_eq!(status, 226);
        assert_eq!(msg, "Transfer complete.");

        let outbuf = writer.into_inner().unwrap();
        let outstr = String::from_utf8(outbuf).unwrap();
        let lines: Vec<&str> = outstr.split("\r\n").collect();

        for line in lines {
            if line.ends_with(" sh") {
                assert!(line.starts_with("-rw-r--r-- 1 owner group"));
            }
        }
    }
}
