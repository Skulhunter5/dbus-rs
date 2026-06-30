#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorName(String);

impl ErrorName {
    pub const MAX_LENGTH: usize = super::MAX_LENGTH;

    pub fn new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        Self::validate(&name).then_some(Self(name))
    }

    fn validate_element_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }

    fn validate(name: &str) -> bool {
        super::validate(name, '.', 2, false, Self::validate_element_char)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ErrorName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<ErrorName> for String {
    fn from(value: ErrorName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use crate::names::ErrorName;

    #[test]
    fn valid() {
        assert!(ErrorName::new("org.freedesktop.DBus.Error").is_some());
    }

    #[test]
    fn valid_with_digits() {
        assert!(ErrorName::new("org.freedesktop.asa2._ac.Error").is_some());
    }

    #[test]
    fn max_length() {
        let mut name_string = String::from("org.freedesktop.Error.");
        name_string.push_str(
            "a".repeat(ErrorName::MAX_LENGTH - name_string.len())
                .as_str(),
        );
        assert!(name_string.len() == ErrorName::MAX_LENGTH);
        assert!(ErrorName::new(name_string).is_some());
    }

    #[test]
    fn not_enough_elements() {
        assert!(ErrorName::new("Error").is_none());
    }

    #[test]
    fn element_starts_with_digit() {
        assert!(ErrorName::new("org.2freedesktop.DBus.Error").is_none());
    }

    #[test]
    fn too_long() {
        let mut name_string = String::from("org.freedesktop.Error.");
        name_string.push_str(
            "a".repeat(ErrorName::MAX_LENGTH - name_string.len() + 1)
                .as_str(),
        );
        assert!(name_string.len() > ErrorName::MAX_LENGTH);
        assert!(ErrorName::new(name_string).is_none());
    }

    #[test]
    fn empty_first_element() {
        assert!(ErrorName::new(".freedesktop.DBus.Error").is_none());
    }

    #[test]
    fn empty_middle_element() {
        assert!(ErrorName::new("org..DBus.Error").is_none());
    }

    #[test]
    fn empty_last_element() {
        assert!(ErrorName::new("org.freedesktop.DBus.").is_none());
    }
}
