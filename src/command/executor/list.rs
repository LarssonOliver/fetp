use std::{
    ffi::OsStr,
    fs::read_dir,
    io::{Read, Write},
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
    time::SystemTime,
};

use chrono::{format::strftime, DateTime, Duration, Local};
use glob::{glob, Paths};
use log::{info, warn};

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
    _start_position: usize,
    _read_stream: Option<&mut dyn Read>,
    write_stream: Option<&mut dyn Write>,
) -> (Status, String) {
    let out_stream = match write_stream {
        Some(stream) => stream,
        None => return (425, "No data connection was established.".to_string()),
    };

    let paths = match read_paths(argument) {
        Ok(paths) => paths,
        Err(out) => return out,
    };

    let outbuf = create_output(&paths);

    match out_stream.write(outbuf.as_bytes()) {
        Ok(bytes) => {
            info!("Wrote {} bytes", bytes);
            (226, "Transfer complete.".to_string())
        }
        Err(error) => {
            warn!("Error while writing data stream: {}", error);
            (426, "Error while sending data.".to_string())
        }
    }
}

fn read_paths(path: &str) -> Result<Vec<PathBuf>, (Status, String)> {
    let res = match read_dir(path) {
        Ok(dir) => dir.filter_map(Result::ok).map(|x| x.path()).collect(),
        Err(ref err) if err.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err((451, "Error reading directory or file.".to_string()))
        }
        Err(_) => extract_from_glob(glob(path).expect("Glob error.")),
    };

    Ok(res)
}

fn extract_from_glob(glob_out: Paths) -> Vec<PathBuf> {
    glob_out.filter_map(Result::ok).collect()
}

fn create_output(paths: &Vec<PathBuf>) -> String {
    paths
        .iter()
        .filter(|x| x.exists())
        .map(create_list_record)
        .fold(String::new(), |acc: String, x: String| {
            format!("{}{}\r\n", acc, x)
        })
}

fn create_list_record(path: &PathBuf) -> String {
    let metadata = path.metadata().expect("Unable to get metadata.");

    // In this context, symlinks are dirs.
    let is_dir = metadata.is_dir() || metadata.is_symlink();
    let size = metadata.size();
    let modified = metadata.modified().expect("Unable to read modified date.");

    const DIR_PREFIX: &str = "drwxr-xr-x 1 owner group";
    const REG_PREFIX: &str = "-rw-r--r-- 1 owner group";

    let prefix = match is_dir {
        true => DIR_PREFIX,
        false => REG_PREFIX,
    };

    let date_string = format_date(modified);

    let name = path.file_name().unwrap().to_str().unwrap();

    format!("{} {:>13} {} {}", prefix, size, date_string, name)
}

fn format_date(time: SystemTime) -> String {
    let now = Local::now();
    let six_months_ago = now.checked_sub_signed(Duration::days(6 * 30)).unwrap();

    let datetime: DateTime<Local> = time.into();
    let older_than_6m = datetime < six_months_ago;

    match older_than_6m {
        false => datetime.format("%b %_d %M:%S"),
        true => datetime.format("%b %_d  %Y"),
    }
    .to_string()
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
        let (status, msg) = data_transfer_func("/usr", 0, None, Some(&mut writer));
        assert_eq!(status, 226);
        assert_eq!(msg, "Transfer complete.");
        let outbuf = writer.into_inner().unwrap();
        let outstr = String::from_utf8(outbuf).unwrap();
        let lines: Vec<&str> = outstr.split("\r\n").collect();

        let dir = fs::read_dir("/usr").unwrap();
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
        assert!(String::from_utf8(outbuf).unwrap().ends_with(" sh\r\n"));
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
            if line.ends_with("usr") {
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
            if line.ends_with("sh") {
                assert!(line.starts_with("-rw-r--r-- 1 owner group"));
            }
        }
    }
}
