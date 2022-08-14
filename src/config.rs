// Default values come from:
// http://sup.xenya.si/sup/info/Juniper/ScreenOS_5.4.0/DocCD_files/Help/5.4.0/ftp_service.htm

// TODO These should be modifiable from command line or environment variables.
pub const MAX_LINE_LENGTH: usize = 1024;
// pub const MAX_USERNAME_LENGTH: usize = 32;
// pub const MAX_PASSWORD_LENGTH: usize = 64;
// pub const MAX_PATH_LENGTH: usize = 512;
// pub const MAX_SITE_STRING_LENGTH: usize = 512;
// pub const MAX_LOGIN_FAILURES_PER_MINUTE: usize = 10;

pub const LISTEN_PORT: u16 = 2121;
pub const LISTEN_ADDR: &str = "127.0.0.1";

pub const NAME_PREFIX: &str = "/";
