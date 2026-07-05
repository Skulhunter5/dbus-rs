// TODO: check that the value is actually a valid signature, i.e. represents a valid type
// see [[https://dbus.freedesktop.org/doc/dbus-specification.html#type-system]] > type codes

use crate::wire_format::MessageReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TypeCode {
    Byte = b'y',
    Boolean = b'b',
    I16 = b'n',
    U16 = b'q',
    I32 = b'i',
    U32 = b'u',
    I64 = b'x',
    U64 = b't',
    Double = b'd',
    String = b's',
    ObjectPath = b'o',
    Signature = b'g',
    Array = b'a',
    Struct = b'r',
    Variant = b'v',
    DictEntry = b'e',
    UnixFd = b'h',
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature(Vec<u8>);

impl Signature {
    pub const MAX_LENGTH: usize = u8::MAX as usize;

    pub const TYPE_CODE_BYTE: u8 = b'y';
    pub const TYPE_CODE_BOOLEAN: u8 = b'b';
    pub const TYPE_CODE_I16: u8 = b'n';
    pub const TYPE_CODE_U16: u8 = b'q';
    pub const TYPE_CODE_I32: u8 = b'i';
    pub const TYPE_CODE_U32: u8 = b'u';
    pub const TYPE_CODE_I64: u8 = b'x';
    pub const TYPE_CODE_U64: u8 = b't';
    pub const TYPE_CODE_DOUBLE: u8 = b'd';
    pub const TYPE_CODE_UNIX_FD: u8 = b'h';
    pub const TYPE_CODE_STRING: u8 = b's';
    pub const TYPE_CODE_OBJECT_PATH: u8 = b'o';
    pub const TYPE_CODE_SIGNATURE: u8 = b'g';
    pub const TYPE_CODE_ARRAY: u8 = b'a';
    pub const TYPE_CODE_STRUCT: u8 = b'r';

    fn validate(signature: &str) -> bool {
        if signature.len() > Self::MAX_LENGTH || signature.is_empty() {
            return false;
        }

        // TODO: replace this with a precise check for valid type codes
        if !signature.is_ascii() {
            return false;
        }

        true
    }

    pub fn as_str(&self) -> &str {
        match str::from_utf8(&self.0) {
            Ok(s) => s,
            Err(_) => unreachable!("signature.0 is generated from valid String in constructor"),
        }
    }

    pub fn is_base_type(&self, type_code: TypeCode) -> bool {
        if self.0.len() > 1 {
            return false;
        }
        let first_code = self.0[0];
        first_code == type_code as u8
    }

    pub fn read_value_bytes<T: byteorder::ByteOrder>(
        &self,
        reader: &mut MessageReader<impl std::io::Read>,
    ) -> std::io::Result<Vec<u8>> {
        todo!();
    }

    pub fn get_value_alignment(&self) -> usize {
        let first_char = self.0[0];
        match first_char {
            b'a' => (),
            _ => todo!(),
        }

        todo!();
    }
}

impl AsRef<str> for Signature {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Signature> for String {
    fn from(value: Signature) -> Self {
        match String::from_utf8(value.0) {
            Ok(s) => s,
            Err(_) => unreachable!("signature.0 is generated from valid String in constructor"),
        }
    }
}

impl TryFrom<String> for Signature {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if Self::validate(&string) {
            Ok(Self(string.into_bytes()))
        } else {
            Err(string)
        }
    }
}

impl<'a> TryFrom<&'a str> for Signature {
    type Error = &'a str;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        if Self::validate(string) {
            Ok(Self(string.to_owned().into_bytes()))
        } else {
            Err(string)
        }
    }
}

impl std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Signature").field(&self.as_str()).finish()
    }
}

#[cfg(test)]
mod test {
    use super::Signature;

    #[test]
    fn valid() {
        assert!(Signature::try_from("test").is_ok());
    }

    #[test]
    fn max_length() {
        let signature_string = "a".repeat(Signature::MAX_LENGTH);
        assert!(Signature::try_from(signature_string).is_ok());
    }

    #[test]
    fn too_long() {
        let signature_string = "a".repeat(Signature::MAX_LENGTH + 1);
        assert!(Signature::try_from(signature_string).is_err());
    }
}
