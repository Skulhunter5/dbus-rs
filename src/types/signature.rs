// TODO: check that the value is actually a valid signature, i.e. represents a valid type
// see [[https://dbus.freedesktop.org/doc/dbus-specification.html#type-system]] > type codes

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature(String);

impl Signature {
    pub const MAX_LENGTH: usize = u8::MAX as usize;

    pub fn new(signature: impl Into<String>) -> Option<Self> {
        let signature = signature.into();
        Self::validate(&signature).then_some(Self(signature))
    }

    fn validate(signature: &str) -> bool {
        if signature.len() > Self::MAX_LENGTH {
            return false;
        }

        true
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Signature {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Signature> for String {
    fn from(value: Signature) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::Signature;

    #[test]
    fn valid() {
        assert!(Signature::new("test").is_some());
    }

    #[test]
    fn max_length() {
        let signature_string = "a".repeat(Signature::MAX_LENGTH);
        assert!(Signature::new(signature_string).is_some());
    }

    #[test]
    fn too_long() {
        let signature_string = "a".repeat(Signature::MAX_LENGTH + 1);
        assert!(Signature::new(signature_string).is_none());
    }
}
