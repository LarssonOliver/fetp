pub mod errors;
pub mod verb;

use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use std::str::{self, FromStr};

use crate::config;
use errors::CommandError;

use self::verb::Verb;

const VERB_LENGTH: usize = 4;

#[derive(Debug)]
pub struct Command {
    pub verb: Verb,
    pub arg: Vec<u8>,
}

pub fn parse(line: &[u8]) -> Result<Command, CommandError> {
    Command::new_from_buffer(&line)
}

impl Command {
    fn new_from_buffer(buffer: &[u8]) -> Result<Command, CommandError> {
        validate_incoming_buffer(&buffer)?;

        let result = Command {
            verb: extract_verb(&buffer)?,
            arg: extract_argument(&buffer),
        };

        info!("Parsed command: {:?} {:?}", result.verb, result.arg);

        Ok(result)
    }
}

fn extract_verb(buffer: &[u8]) -> Result<Verb, CommandError> {
    let string = str::from_utf8(&buffer[..VERB_LENGTH]).unwrap();
    match Verb::from_str(string) {
        Ok(verb) => Ok(verb),
        Err(err_str) => Err(CommandError(err_str)),
    }
}

fn extract_argument(buffer: &[u8]) -> Vec<u8> {
    let mut control_char_count = 0;
    while buffer
        .get(buffer.len() - control_char_count - 1)
        .unwrap()
        .is_ascii_control()
    {
        control_char_count += 1;
    }

    if buffer.len() <= VERB_LENGTH + control_char_count {
        return Vec::new();
    }

    // + 1 is for the space between the verb and the argument.
    buffer[VERB_LENGTH + 1..buffer.len() - control_char_count].to_vec()
}

fn validate_incoming_buffer(buffer: &[u8]) -> Result<(), CommandError> {
    validate_incoming_buffer_length(&buffer)?;
    validate_incoming_buffer_format(&buffer)?;
    Ok(())
}

fn validate_incoming_buffer_format(buffer: &[u8]) -> Result<(), CommandError> {
    lazy_static! {
        static ref MATCHER: Regex = Regex::new(r"^[A-Za-z]{4}( .*)?\r?\n?$").unwrap();
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

    match str::from_utf8(buffer) {
        Ok(text) => Ok(text.to_string()),
        Err(_) => Err(CommandError::default()),
    }
}

fn validate_incoming_buffer_length(buffer: &[u8]) -> Result<(), CommandError> {
    if buffer.len() < VERB_LENGTH {
        return Err(CommandError::default());
    }

    if buffer.len() > config::MAX_LINE_LENGTH {
        return Err(CommandError(format!(
            "Command too long, max length is {} bytes",
            config::MAX_LINE_LENGTH
        )));
    }

    Ok(())
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
        assert_eq!(result.arg, "anonymous".as_bytes());
    }

    #[test]
    fn test_command_from_valid_without_args() {
        let com = "USER\r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "".as_bytes());
        let com = "USER \r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "".as_bytes());
    }

    #[test]
    fn test_command_from_valid_without_crlf() {
        let com = "USER anonymous";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "anonymous".as_bytes());
        let com = "USER ";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "".as_bytes());
    }

    #[test]
    fn test_command_from_valid_without_cr() {
        let com = "USER anonymous\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "anonymous".as_bytes());
        let com = "USER\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, Verb::USER);
        assert_eq!(result.arg, "".as_bytes());
    }

    #[test]
    fn test_command_from_non_ascii() {
        let com = "USER ö\r\n";
        let result = parse(com.as_bytes());
        assert_eq!(result.is_err(), true);
    }
}
