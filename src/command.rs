mod errors;

use errors::*;

use lazy_static::lazy_static;
use regex::Regex;
use std::str;

use crate::config;

const VERB_LENGTH: usize = 4;

#[derive(Debug, PartialEq)]
pub struct Command {
    verb: [u8; VERB_LENGTH],
    arg: Vec<u8>,
}

// pub fn parse(line: &[u8]) {}

impl Command {
    fn new_from_buffer(buffer: &[u8]) -> Result<Command, CommandError> {
        validate_incoming_buffer(&buffer)?;

        let result = Command {
            verb: extract_verb(&buffer),
            arg: extract_argument(&buffer),
        };

        return Ok(result);
    }
}

fn extract_verb(buffer: &[u8]) -> [u8; VERB_LENGTH] {
    let mut result = [0; VERB_LENGTH];
    result.copy_from_slice(&buffer[0..VERB_LENGTH]);
    return result;
}

fn extract_argument(buffer: &[u8]) -> Vec<u8> {
    // + 1 is for the space between the verb and the argument.
    buffer[VERB_LENGTH + 1..buffer.len() - 2].to_vec()
}

fn validate_incoming_buffer(buffer: &[u8]) -> Result<(), CommandError> {
    validate_incoming_buffer_length(&buffer)?;
    validate_incoming_buffer_format(&buffer)?;
    Ok(())
}

fn validate_incoming_buffer_format(buffer: &[u8]) -> Result<(), CommandError> {
    lazy_static! {
        static ref MATCHER: Regex = Regex::new(r"^[A-Za-z]{4}( .*)?\r?\n$").unwrap();
    }

    let text = utf8_buffer_to_string(buffer)?;

    if !MATCHER.is_match(&text) {
        return Err(CommandError(String::from("Invalid command format")));
    }

    Ok(())
}

fn utf8_buffer_to_string(buffer: &[u8]) -> Result<String, CommandError> {
    // if !buffer.is_ascii() {
    //     return Err(CommandError(String::from(
    //         "Command contains non-ASCII characters",
    //     )))
    // }

    match str::from_utf8(buffer) {
        Ok(text) => Ok(text.to_string()),
        Err(_) => Err(CommandError::default()),
    }
}

fn validate_incoming_buffer_length(buffer: &[u8]) -> Result<(), CommandError> {
    if buffer.len() < VERB_LENGTH + 1 {
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
        let result = Command::new_from_buffer(com.as_bytes());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_command_from_valid_with_args() {
        let com = "USER anonymous\r\n";
        let result = Command::new_from_buffer(com.as_bytes());
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result.verb, "USER".as_bytes());
        assert_eq!(result.arg, "anonymous".as_bytes());
    }
}
