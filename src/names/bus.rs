#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BusName(String);

impl BusName {
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
        c.is_ascii_alphanumeric() || *c == '_' || *c == '-'
    }

    fn validate(name: &str) -> Result<(), String> {
        if name.len() > Self::MAX_LENGTH {
            return Err(format!(
                "too long: has {} but max allowed is {}",
                name.len(),
                Self::MAX_LENGTH
            ));
        }

        let Some(first_char) = name.chars().next() else {
            return Err("empty".to_owned());
        };
        let (name, elements_can_start_with_digit) = if first_char == ':' {
            (&name[(':'.len_utf8())..], true)
        } else {
            (name, false)
        };
        super::validate(
            name,
            Self::SEPARATOR,
            Self::MIN_ELEMENT_COUNT,
            elements_can_start_with_digit,
            Self::validate_element_char,
        )
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for BusName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value))
    }
}

impl<'a> TryFrom<&'a str> for BusName {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value.to_owned()))
    }
}

impl AsRef<str> for BusName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<BusName> for String {
    fn from(value: BusName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::names::BusName;

    #[test]
    fn unique() {
        assert!(BusName::new(":1.2").is_some());
    }

    #[test]
    fn unique_with_letters() {
        assert!(BusName::new(":org.freedesktop.DBus").is_some());
    }

    #[test]
    fn well_known() {
        assert!(BusName::new("org.freedesktop.DBus").is_some());
    }

    #[test]
    fn well_known_2() {
        assert!(BusName::new("org.freedesktop.DBus.a2._adf").is_some());
    }

    #[test]
    fn well_known_element_starts_with_digit() {
        assert!(BusName::new("org.2freedesktop.DBus").is_none());
    }

    #[test]
    fn unique_max_length() {
        let mut name_string = String::from(":1.0.");
        name_string.push_str("2".repeat(BusName::MAX_LENGTH - name_string.len()).as_str());
        assert!(name_string.len() == BusName::MAX_LENGTH);
        assert!(BusName::new(name_string).is_some());
    }

    #[test]
    fn well_known_max_length() {
        let mut name_string = String::from("org.freedesktop.");
        name_string.push_str("a".repeat(BusName::MAX_LENGTH - name_string.len()).as_str());
        assert!(name_string.len() == BusName::MAX_LENGTH);
        assert!(BusName::new(name_string).is_some());
    }

    #[test]
    fn unique_not_enough_elements() {
        assert!(BusName::new(":1").is_none());
    }

    #[test]
    fn well_known_not_enough_elements() {
        assert!(BusName::new("org").is_none());
    }

    #[test]
    fn unique_too_long() {
        let mut name_string = String::from(":1.0.");
        name_string.push_str(
            "2".repeat(BusName::MAX_LENGTH - name_string.len() + 1)
                .as_str(),
        );
        assert!(name_string.len() > BusName::MAX_LENGTH);
        assert!(BusName::new(name_string).is_none());
    }

    #[test]
    fn well_known_too_long() {
        let mut name_string = String::from("org.freedesktop.");
        name_string.push_str(
            "a".repeat(BusName::MAX_LENGTH - name_string.len() + 1)
                .as_str(),
        );
        assert!(name_string.len() > BusName::MAX_LENGTH);
        assert!(BusName::new(name_string).is_none());
    }

    #[test]
    fn empty_first_element() {
        assert!(BusName::new(".freedesktop.DBus").is_none());
    }

    #[test]
    fn empty_middle_element() {
        assert!(BusName::new("org..DBus").is_none());
    }

    #[test]
    fn empty_last_element() {
        assert!(BusName::new("org.freedesktop.").is_none());
    }
}
