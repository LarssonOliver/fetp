use std::str::FromStr;

use log::warn;

use super::executor::acct::acct_command_executor;
use super::executor::mode::mode_command_executor;
use super::executor::pass::pass_command_executor;
use super::executor::r#type::type_command_executor;
use super::executor::stru::stru_command_executor;
use super::executor::user::user_command_executor;
use super::executor::Executor;

/// Available FTP commands.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Verb {
    USER,
    PASS,
    ACCT,
    TYPE,
    STRU,
    MODE,
}

impl FromStr for Verb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper_s = s.to_uppercase();
        match upper_s.as_str() {
            "USER" => Ok(Verb::USER),
            "PASS" => Ok(Verb::PASS),
            "ACCT" => Ok(Verb::ACCT),
            "TYPE" => Ok(Verb::TYPE),
            "STRU" => Ok(Verb::STRU),
            "MODE" => Ok(Verb::MODE),
            _ => {
                warn!("Unknown verb: {}", s);
                Err(format!("Unknown verb: {}", s))
            }
        }
    }
}

impl Verb {
    pub(super) fn executor(&self) -> Executor {
        match self {
            Verb::USER => user_command_executor,
            Verb::PASS => pass_command_executor,
            Verb::ACCT => acct_command_executor,
            Verb::TYPE => type_command_executor,
            Verb::STRU => stru_command_executor,
            Verb::MODE => mode_command_executor,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::executor::{r#type::type_command_executor, stru::stru_command_executor};

    use super::*;

    #[test]
    fn test_valid_parse() {
        let user = "USER";
        let pass = "PASS";
        assert!(Verb::from_str(user).is_ok());
        assert!(Verb::from_str(pass).is_ok());
        assert_eq!(Verb::from_str(user).unwrap(), Verb::USER);
        assert_eq!(Verb::from_str(pass).unwrap(), Verb::PASS);
    }

    #[test]
    fn test_valid_any_case() {
        let verbs = ["user", "USER", "User", "uSER", "UsEr", "uSeR"];
        for verb in verbs.iter() {
            assert!(Verb::from_str(verb).is_ok());
            assert_eq!(Verb::from_str(verb).unwrap(), Verb::USER);
        }
    }

    #[test]
    fn test_fail_invalid() {
        let invalid = "foo";
        assert!(Verb::from_str(invalid).is_err());
    }

    #[test]
    fn test_fail_on_empty() {
        let empty = "";
        assert!(Verb::from_str(empty).is_err());
    }

    #[test]
    fn test_executor_mapping() {
        assert_eq!(
            Verb::USER.executor() as usize,
            user_command_executor as usize
        );
        assert_eq!(
            Verb::PASS.executor() as usize,
            pass_command_executor as usize
        );
        assert_eq!(
            Verb::ACCT.executor() as usize,
            acct_command_executor as usize
        );
        assert_eq!(
            Verb::TYPE.executor() as usize,
            type_command_executor as usize
        );
        assert_eq!(
            Verb::STRU.executor() as usize,
            stru_command_executor as usize
        );
        assert_eq!(
            Verb::MODE.executor() as usize,
            mode_command_executor as usize
        );
    }
}
