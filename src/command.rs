pub(crate) mod errors;
mod executor;
pub(crate) mod verb;

use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use std::str::{self, FromStr};

use crate::{config, session::sessionstate::SessionState};
use errors::CommandError;

use self::verb::Verb;

#[derive(Debug)]
pub(crate) struct Command {
    pub verb: Verb,
    pub arg: String,
}

pub(crate) fn parse(line: &[u8]) -> Result<Command, CommandError> {
    Command::new_from_buffer(&line)
}

impl Command {
    fn new_from_buffer(buffer: &[u8]) -> Result<Command, CommandError> {
        validate_incoming_buffer(&buffer)?;

        let verb = extract_verb(&buffer)?;
        let arg = extract_argument(&buffer, format!("{:?}", verb).len());
        let result = Command { verb, arg };

        info!("Parsed command: {:?} {:?}", result.verb, result.arg);

        Ok(result)
    }

    pub(crate) fn execute(
        &self,
        current_state: &SessionState,
    ) -> Result<executor::ExecutionResult, errors::ExecutionError> {
        let executor = self.verb.executor();
        executor(current_state, self.arg.as_str())
    }
}

fn validate_incoming_buffer(buffer: &[u8]) -> Result<(), CommandError> {
    validate_incoming_buffer_length(&buffer)?;
    validate_incoming_buffer_format(&buffer)
}

fn validate_incoming_buffer_length(buffer: &[u8]) -> Result<(), CommandError> {
    if buffer.len() > config::MAX_LINE_LENGTH {
        return Err(CommandError(format!(
            "Command too long, max length is {} bytes",
            config::MAX_LINE_LENGTH
        )));
    }

    Ok(())
}

fn validate_incoming_buffer_format(buffer: &[u8]) -> Result<(), CommandError> {
    lazy_static! {
        static ref MATCHER: Regex = Regex::new(r"^[A-Za-z]+( .*)?\r?\n?$").unwrap();
    }

    let text = utf8_buffer_to_string(buffer)?;

    if !MATCHER.is_match(&text) {
        return Err(CommandError(String::from("Invalid command format")));
    }

    Ok(())
}

fn utf8_buffer_to_string(buffer: &[u8]) -> Result<String, CommandError> {
    if !buffer.is_ascii() {
        return Err(CommandError(String::from(
            "Command contains non-ASCII characters",
        )));
    }

    Ok(str::from_utf8(buffer).unwrap().to_string())
}

fn extract_verb(buffer: &[u8]) -> Result<Verb, CommandError> {
    let verb_length = buffer
        .iter()
        .position(|c| c == &b' ' || c == &b'\r' || c == &b'\n')
        .unwrap_or(buffer.len());

    let string = str::from_utf8(&buffer[..verb_length]).unwrap();

    match Verb::from_str(string) {
        Ok(verb) => Ok(verb),
        Err(err_str) => Err(CommandError(err_str)),
    }
}

fn extract_argument(buffer: &[u8], verb_length: usize) -> String {
    let mut control_char_count = 0;
    while buffer
        .get(buffer.len() - control_char_count - 1)
        .unwrap()
        .is_ascii_control()
    {
        control_char_count += 1;
    }

    if buffer.len() <= verb_length + control_char_count {
        return String::new();
    }

    // + 1 is for the space between the verb and the argument.
    let vec = buffer[verb_length + 1..buffer.len() - control_char_count].to_vec();

    String::from_utf8(vec).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_empty_buffer() {
        let com = "";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_command_from_valid_with_args() {
        let com = "USER anonymous\r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "anonymous");
    }

    #[test]
    fn test_command_from_valid_without_args() {
        let com = "USER\r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "");
        let com = "USER \r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "");
    }

    #[test]
    fn test_command_from_valid_without_crlf() {
        let com = "USER anonymous";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "anonymous");
        let com = "USER ";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "");
    }

    #[test]
    fn test_command_from_valid_without_cr() {
        let com = "USER anonymous\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "anonymous");
        let com = "USER\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "");
    }

    #[test]
    fn test_command_from_non_ascii() {
        let com = "USER ö\r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_invalid_verb() {
        let com = b"XYZZ\r\n";
        let result = parse(com);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format() {
        let com = b"foobar";
        let result = parse(com);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_length() {
        let start = b"USER ";
        let mut com = [0; config::MAX_LINE_LENGTH + 1];

        com[..start.len()].copy_from_slice(start);

        let result = parse(com.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_executor() {
        let com = b"USER foo\t\n";
        let result = parse(com).unwrap();
        let state = SessionState::default();
        let result = result.execute(&state);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status, 331);
        assert!(result.new_state.is_some());
        let new_state = result.new_state.unwrap();
        assert_eq!(new_state.user, Some("foo".to_string()));
        assert_eq!(new_state.is_authenticated, false);
    }
}
