use std::str::FromStr;

use log::warn;

use super::executor::acct::acct_command_executor;
use super::executor::cdup::cdup_command_executor;
use super::executor::cwd::cwd_command_executor;
use super::executor::help::help_command_executor;
use super::executor::mode::mode_command_executor;
use super::executor::noop::noop_command_executor;
use super::executor::pass::pass_command_executor;
use super::executor::pasv::pasv_command_executor;
use super::executor::port::port_command_executor;
use super::executor::pwd::pwd_command_executor;
use super::executor::quit::quit_command_executor;
use super::executor::r#type::type_command_executor;
use super::executor::rest::rest_command_executor;
use super::executor::retr::retr_command_executor;
use super::executor::stat::stat_command_executor;
use super::executor::stru::stru_command_executor;
use super::executor::syst::syst_command_executor;
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
    PWD,
    XPWD,
    CWD,
    XCWD,
    CDUP,
    XCUP,
    PASV,
    PORT,
    REST,
    RETR,
    SYST,
    STAT,
    HELP,
    NOOP,
    QUIT,
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
            "PWD" => Ok(Verb::PWD),
            "XPWD" => Ok(Verb::XPWD),
            "CWD" => Ok(Verb::CWD),
            "XCWD" => Ok(Verb::XCWD),
            "CDUP" => Ok(Verb::CDUP),
            "XDUP" => Ok(Verb::XCUP),
            "PASV" => Ok(Verb::PASV),
            "PORT" => Ok(Verb::PORT),
            "REST" => Ok(Verb::REST),
            "RETR" => Ok(Verb::RETR),
            "SYST" => Ok(Verb::SYST),
            "STAT" => Ok(Verb::STAT),
            "HELP" => Ok(Verb::HELP),
            "NOOP" => Ok(Verb::NOOP),
            "QUIT" => Ok(Verb::QUIT),
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
            Verb::PWD | Verb::XPWD => pwd_command_executor,
            Verb::CWD | Verb::XCWD => cwd_command_executor,
            Verb::CDUP | Verb::XCUP => cdup_command_executor,
            Verb::PASV => pasv_command_executor,
            Verb::PORT => port_command_executor,
            Verb::REST => rest_command_executor,
            Verb::RETR => retr_command_executor,
            Verb::SYST => syst_command_executor,
            Verb::STAT => stat_command_executor,
            Verb::HELP => help_command_executor,
            Verb::NOOP => noop_command_executor,
            Verb::QUIT => quit_command_executor,
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(Verb::PWD.executor() as usize, pwd_command_executor as usize);
        assert_eq!(
            Verb::XPWD.executor() as usize,
            pwd_command_executor as usize
        );
        assert_eq!(Verb::CWD.executor() as usize, cwd_command_executor as usize);
        assert_eq!(
            Verb::XCWD.executor() as usize,
            cwd_command_executor as usize
        );
        assert_eq!(
            Verb::CDUP.executor() as usize,
            cdup_command_executor as usize
        );
        assert_eq!(
            Verb::XCUP.executor() as usize,
            cdup_command_executor as usize
        );
        assert_eq!(
            Verb::PASV.executor() as usize,
            pasv_command_executor as usize
        );
        assert_eq!(
            Verb::RETR.executor() as usize,
            retr_command_executor as usize
        );
        assert_eq!(
            Verb::REST.executor() as usize,
            rest_command_executor as usize
        );
        assert_eq!(
            Verb::PORT.executor() as usize,
            port_command_executor as usize
        );
        assert_eq!(
            Verb::SYST.executor() as usize,
            syst_command_executor as usize
        );
        assert_eq!(
            Verb::STAT.executor() as usize,
            stat_command_executor as usize
        );
        assert_eq!(
            Verb::HELP.executor() as usize,
            help_command_executor as usize
        );
        assert_eq!(
            Verb::NOOP.executor() as usize,
            noop_command_executor as usize
        );
        assert_eq!(
            Verb::QUIT.executor() as usize,
            quit_command_executor as usize
        );
    }
}
