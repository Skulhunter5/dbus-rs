use crate::names::MAX_NAME_LENGTH;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BusName(String);

impl BusName {
    pub fn new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        Self::validate(&name).then(|| Self(name))
    }

    fn validate_element_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_' || *c == '-'
    }

    fn validate(name: &str) -> bool {
        if name.len() > MAX_NAME_LENGTH {
            return false;
        }

        let Some(first_char) = name.chars().next() else {
            return false;
        };
        if first_char == ':' {
            let name = &name[(':'.len_utf8())..];
            super::validate(name, '.', 2, true, Self::validate_element_char)
        } else {
            super::validate(name, '.', 2, false, Self::validate_element_char)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::names::{BusName, MAX_NAME_LENGTH};

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
        name_string.push_str(
            std::iter::repeat('2')
                .take(MAX_NAME_LENGTH - name_string.len())
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() == MAX_NAME_LENGTH);
        assert!(BusName::new(name_string).is_some());
    }

    #[test]
    fn well_known_max_length() {
        let mut name_string = String::from("org.freedesktop.");
        name_string.push_str(
            std::iter::repeat('a')
                .take(MAX_NAME_LENGTH - name_string.len())
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() == MAX_NAME_LENGTH);
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
            std::iter::repeat('2')
                .take(MAX_NAME_LENGTH - name_string.len() + 1)
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() > MAX_NAME_LENGTH);
        assert!(BusName::new(name_string).is_none());
    }

    #[test]
    fn well_known_too_long() {
        let mut name_string = String::from("org.freedesktop.");
        name_string.push_str(
            std::iter::repeat('a')
                .take(MAX_NAME_LENGTH - name_string.len() + 1)
                .collect::<String>()
                .as_str(),
        );
        assert!(name_string.len() > MAX_NAME_LENGTH);
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
