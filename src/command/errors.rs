use std::fmt;

use super::verb::Verb;

#[derive(Debug, Default)]
pub struct CommandError(pub String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "Invalid command")
        } else {
            write!(f, "Invalid command: {}", &self.0)
        }
    }
}

#[derive(Debug)]
pub struct ExecutionError(pub Verb, pub String);

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1.is_empty() {
            write!(f, "Error when executing command {:?}", self.0)
        } else {
            write!(f, "Error when executing command {:?}: {}", self.0, &self.1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_no_message() {
        let err = CommandError::default();
        let s = format!("{}", err);
        assert_eq!(s, "Invalid command");
    }

    #[test]
    fn test_fmt_message() {
        let err = CommandError("foo".to_string());
        let s = format!("{}", err);
        assert_eq!(s, "Invalid command: foo");
    }

    #[test]
    fn test_fmt_execution_error_no_message() {
        let err = ExecutionError(Verb::USER, "".to_string());
        let s = format!("{}", err);
        assert_eq!(s, "Error when executing command USER");
    }

    #[test]
    fn test_fmt_execution_error_message() {
        let err = ExecutionError(Verb::USER, "foo".to_string());
        let s = format!("{}", err);
        assert_eq!(s, "Error when executing command USER: foo");
    }
}
