use crate::names::MAX_NAME_LENGTH;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemberName(String);

impl MemberName {
    pub fn new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        Self::validate(&name).then(|| Self(name))
    }

    fn validate_element_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }

    fn validate(name: &str) -> bool {
        if name.is_empty() || name.len() > MAX_NAME_LENGTH {
            return false;
        }
        super::validate_element(name, false, Self::validate_element_char)
    }
}

#[cfg(test)]
mod test {
    use crate::names::{MAX_NAME_LENGTH, MemberName};

    #[test]
    fn valid_1() {
        assert!(MemberName::new("GetItems").is_some());
    }

    #[test]
    fn valid_2() {
        assert!(MemberName::new("ItemsChanged").is_some());
    }

    #[test]
    fn valid_with_digits() {
        assert!(MemberName::new("GetSomething2_test").is_some());
    }

    #[test]
    fn max_length() {
        let name_string = std::iter::repeat('a')
            .take(MAX_NAME_LENGTH)
            .collect::<String>();
        assert!(name_string.len() == MAX_NAME_LENGTH);
        assert!(MemberName::new(name_string).is_some());
    }

    #[test]
    fn starts_with_digit() {
        assert!(MemberName::new("2SomethingElse").is_none());
    }

    #[test]
    fn too_long() {
        let name_string = std::iter::repeat('a')
            .take(MAX_NAME_LENGTH + 1)
            .collect::<String>();
        assert!(name_string.len() > MAX_NAME_LENGTH);
        assert!(MemberName::new(name_string).is_none());
    }
}
