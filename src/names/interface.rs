#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceName(String);

impl InterfaceName {
    pub const MAX_LENGTH: usize = super::MAX_LENGTH;
    pub const SEPARATOR: char = '.';
    pub const MIN_ELEMENT_COUNT: usize = 2;

    pub fn new(name: impl Into<String> + AsRef<str>) -> Option<Self> {
        match Self::validate(name.as_ref()) {
            Ok(()) => Some(Self(name.into())),
            Err(_) => None,
        }
    }

    fn validate_element_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }

    fn validate(name: &str) -> Result<(), String> {
        super::validate(
            name,
            Self::SEPARATOR,
            Self::MIN_ELEMENT_COUNT,
            false,
            Self::validate_element_char,
        )
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for InterfaceName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value))
    }
}

impl<'a> TryFrom<&'a str> for InterfaceName {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value.to_owned()))
    }
}

impl AsRef<str> for InterfaceName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<InterfaceName> for String {
    fn from(value: InterfaceName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::names::InterfaceName;

    #[test]
    fn valid() {
        assert!(InterfaceName::new("org.freedesktop.DBus").is_some());
    }

    #[test]
    fn valid_with_digits() {
        assert!(InterfaceName::new("org.freedesktop.asa2._ac").is_some());
    }

    #[test]
    fn max_length() {
        let mut name_string = String::from("org.freedesktop.");
        name_string.push_str(
            "a".repeat(InterfaceName::MAX_LENGTH - name_string.len())
                .as_str(),
        );
        assert!(name_string.len() == InterfaceName::MAX_LENGTH);
        assert!(InterfaceName::new(name_string).is_some());
    }

    #[test]
    fn not_enough_elements() {
        assert!(InterfaceName::new("org").is_none());
    }

    #[test]
    fn element_starts_with_digit() {
        assert!(InterfaceName::new("org.2freedesktop.DBus").is_none());
    }

    #[test]
    fn too_long() {
        let mut name_string = String::from("org.freedesktop.");
        name_string.push_str(
            "a".repeat(InterfaceName::MAX_LENGTH - name_string.len() + 1)
                .as_str(),
        );
        assert!(name_string.len() > InterfaceName::MAX_LENGTH);
        assert!(InterfaceName::new(name_string).is_none());
    }

    #[test]
    fn empty_first_element() {
        assert!(InterfaceName::new(".freedesktop.DBus").is_none());
    }

    #[test]
    fn empty_middle_element() {
        assert!(InterfaceName::new("org..DBus").is_none());
    }

    #[test]
    fn empty_last_element() {
        assert!(InterfaceName::new("org.freedesktop.").is_none());
    }
}
