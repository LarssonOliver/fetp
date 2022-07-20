use std::fmt;

#[derive(Debug, Default)]
pub struct CommandError(pub(crate) String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            writeln!(f, "Invalid command")
        } else {
            writeln!(f, "Invalid command: {}", &self.0)
        }
    }
}
