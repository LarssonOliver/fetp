// TODO Expand this to allow for user authentication.
pub fn validate(username: &str, _password: &str) -> bool {
    if username == "anonymous" {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_false() {
        assert_eq!(validate("", ""), false);
    }

    #[test]
    fn test_validate_anonymous_true() {
        assert_eq!(validate("anonymous", ""), true);
        assert_eq!(validate("anonymous", "foobar"), true);
    }

    #[test]
    fn test_validate_user_false() {
        assert_eq!(validate("user", ""), false);
        assert_eq!(validate("user", "foobar"), false);
    }
}
