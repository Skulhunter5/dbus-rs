#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemberName(String);

impl MemberName {
    pub const MAX_LENGTH: usize = super::MAX_LENGTH;

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
        if name.is_empty() {
            return Err("empty".to_owned());
        }
        if name.len() > Self::MAX_LENGTH {
            return Err(format!(
                "too long: has {} but max allowed is {}",
                name.len(),
                Self::MAX_LENGTH
            ));
        }
        super::validate_element(name, false, Self::validate_element_char)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for MemberName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value))
    }
}

impl<'a> TryFrom<&'a str> for MemberName {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::validate(value).map(|_| Self(value.to_owned()))
    }
}

impl AsRef<str> for MemberName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<MemberName> for String {
    fn from(value: MemberName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::names::MemberName;

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
        assert!(MemberName::new("a".repeat(MemberName::MAX_LENGTH)).is_some());
    }

    #[test]
    fn starts_with_digit() {
        assert!(MemberName::new("2SomethingElse").is_none());
    }

    #[test]
    fn too_long() {
        assert!(MemberName::new("a".repeat(MemberName::MAX_LENGTH + 1)).is_none());
    }

    #[test]
    fn empty() {
        assert!(MemberName::new("").is_none());
    }

    #[test]
    fn period() {
        assert!(MemberName::new(".").is_none());
    }

    #[test]
    fn multiple_elements() {
        assert!(MemberName::new("org.freedesktop").is_none());
    }
}
