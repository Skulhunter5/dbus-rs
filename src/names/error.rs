#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorName(String);

impl ErrorName {
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
    use crate::names::{ErrorName, MAX_NAME_LENGTH};

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
            std::iter::repeat('a')
                .take(MAX_NAME_LENGTH - name_string.len())
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() == MAX_NAME_LENGTH);
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
            std::iter::repeat('a')
                .take(MAX_NAME_LENGTH - name_string.len() + 1)
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() > MAX_NAME_LENGTH);
        assert!(ErrorName::new(name_string).is_none());
    }
}
