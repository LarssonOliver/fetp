use std::{net::TcpListener, path::PathBuf};

use crate::{command::verb::Verb, config};

pub(crate) struct SessionState {
    pub(crate) user: Option<String>,
    pub(crate) is_authenticated: bool,
    pub(crate) previous_command: Option<Verb>,
    pub(crate) binary_flag: bool,
    pub(crate) name_prefix: PathBuf,
    pub(crate) has_greeted: bool,

    pub(crate) data_listener: Option<TcpListener>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
            previous_command: None,
            binary_flag: false,
            has_greeted: false,
            name_prefix: PathBuf::from(config::NAME_PREFIX),
            data_listener: None,
        }
    }
}

impl Clone for SessionState {
    fn clone(&self) -> Self {
        Self {
            user: self.user.clone(),
            is_authenticated: self.is_authenticated,
            previous_command: self.previous_command.clone(),
            binary_flag: self.binary_flag,
            has_greeted: self.has_greeted,
            name_prefix: self.name_prefix.clone(),
            data_listener: match self.data_listener {
                Some(ref listener) => {
                    Some(listener.try_clone().expect("Failed to clone data listener"))
                }
                None => None,
            },
        }
    }
}
