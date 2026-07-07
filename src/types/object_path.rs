use crate::wire_format::{WireFormatRead, WireFormatType, WireFormatWrite};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectPath(String);

impl ObjectPath {
    pub const SEPARATOR: char = '/';

    pub fn new(object_path: impl Into<String> + AsRef<str>) -> Option<Self> {
        match Self::validate(object_path.as_ref()) {
            Ok(()) => Some(Self(object_path.into())),
            Err(_) => None,
        }
    }

    fn validate_element_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
    }

    fn validate_element(element: &str) -> Result<(), String> {
        if element.is_empty() {
            return Err("empty element".to_owned());
        }
        if let Some(invalid_char) = element.find(|c| !Self::validate_element_char(&c)) {
            return Err(format!("invalid character: {:?}", invalid_char));
        }

        Ok(())
    }

    fn validate(object_path: &str) -> Result<(), String> {
        if Some(Self::SEPARATOR) != object_path.chars().next() {
            return Err(format!("has to start with {:?}", Self::SEPARATOR));
        };
        let object_path = &object_path[(Self::SEPARATOR.len_utf8())..];
        if object_path.is_empty() {
            // root path
            return Ok(());
        }

        for (element, error) in object_path
            .split(Self::SEPARATOR)
            .map(|element| (element, Self::validate_element(element).err()))
        {
            if let Some(error) = error {
                return Err(format!("invalid element ({:?}): {}", element, error));
            }
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for ObjectPath {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value))
    }
}

impl<'a> TryFrom<&'a str> for ObjectPath {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::validate(&value).map(|_| Self(value.to_owned()))
    }
}

impl WireFormatType for ObjectPath {
    const ALIGNMENT: usize = <String as WireFormatType>::ALIGNMENT;
}

impl WireFormatRead for ObjectPath {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        let object_path_string = reader.read::<T, String>()?;
        match Self::validate(&object_path_string) {
            Ok(()) => Ok(Self(object_path_string)),
            Err(error) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid object path ({:?}): {}", object_path_string, error),
            )),
        }
    }
}

impl WireFormatWrite for ObjectPath {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, _>(&self.0)
    }
}

impl AsRef<str> for ObjectPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<ObjectPath> for String {
    fn from(value: ObjectPath) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::types::ObjectPath;

    #[test]
    fn valid() {
        assert!(ObjectPath::new("/org/freedesktop/DBus").is_some());
    }

    #[test]
    fn valid_with_digits() {
        assert!(ObjectPath::new("/org/freedesktop/2asa2/_ac").is_some());
    }

    #[test]
    fn valid_very_long() {
        let mut path_string = String::from("/org/freedesktop/DBus");
        path_string.push_str("/something".repeat(200).as_str());
        assert!(ObjectPath::new(path_string).is_some());
    }

    #[test]
    fn valid_root() {
        assert!(ObjectPath::new("/").is_some());
    }

    #[test]
    fn empty_element() {
        assert!(ObjectPath::new("/org//DBus").is_none());
    }

    #[test]
    fn trailing_slash() {
        assert!(ObjectPath::new("/org/freedesktop/DBus/").is_none());
    }

    #[test]
    fn invalid_character() {
        assert!(ObjectPath::new("/org/free desktop/DBus").is_none());
    }

    #[test]
    fn trailing_double_slash() {
        assert!(ObjectPath::new("/org/freedesktop//").is_none());
    }
}
