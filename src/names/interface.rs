#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceName(String);

impl InterfaceName {
    pub fn new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        Self::validate(&name).then(|| Self(name))
    }

    fn validate_element_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }

    fn validate(name: &str) -> bool {
        super::validate(name, '.', 2, false, Self::validate_element_char)
    }
}

#[cfg(test)]
mod test {
    use crate::names::{InterfaceName, MAX_NAME_LENGTH};

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
            std::iter::repeat('a')
                .take(MAX_NAME_LENGTH - name_string.len())
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() == MAX_NAME_LENGTH);
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
            std::iter::repeat('a')
                .take(MAX_NAME_LENGTH - name_string.len() + 1)
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() > MAX_NAME_LENGTH);
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
