use std::fmt;

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
}
