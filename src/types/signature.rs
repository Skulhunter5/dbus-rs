// TODO: check that the value is actually a valid signature, i.e. represents a valid type
// see [[https://dbus.freedesktop.org/doc/dbus-specification.html#type-system]] > type codes

use byteorder::ByteOrder;

use crate::{
    types::{ObjectPath, TypeCode, Value, Variant},
    wire_format::MessageReader,
};

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Signature {
// }

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature(Vec<u8>);

impl Signature {
    pub const MAX_LENGTH: usize = u8::MAX as usize;

    const VALID_CHARS: &[char] = &[
        'y', 'b', 'n', 'q', 'i', 'u', 'x', 't', 'd', 's', 'o', 'g', 'a', 'v', 'h',
    ];

    pub fn new_boolean() -> Self {
        Self(vec![TypeCode::Boolean as u8])
    }

    pub fn new_byte() -> Self {
        Self(vec![TypeCode::Byte as u8])
    }

    pub fn new_i16() -> Self {
        Self(vec![TypeCode::I16 as u8])
    }

    pub fn new_u16() -> Self {
        Self(vec![TypeCode::U16 as u8])
    }

    pub fn new_i32() -> Self {
        Self(vec![TypeCode::I32 as u8])
    }

    pub fn new_u32() -> Self {
        Self(vec![TypeCode::U32 as u8])
    }

    pub fn new_i64() -> Self {
        Self(vec![TypeCode::I64 as u8])
    }

    pub fn new_u64() -> Self {
        Self(vec![TypeCode::U64 as u8])
    }

    pub fn new_double() -> Self {
        Self(vec![TypeCode::Double as u8])
    }

    pub fn new_string() -> Self {
        Self(vec![TypeCode::String as u8])
    }

    pub fn new_object_path() -> Self {
        Self(vec![TypeCode::ObjectPath as u8])
    }

    pub fn new_signature() -> Self {
        Self(vec![TypeCode::Signature as u8])
    }

    pub fn new_variant() -> Self {
        Self(vec![TypeCode::Variant as u8])
    }

    pub fn new_unix_fd() -> Self {
        Self(vec![TypeCode::UnixFd as u8])
    }

    fn validate(signature: &str) -> Result<(), String> {
        if signature.is_empty() {
            return Err("empty".to_owned());
        }
        if signature.len() > Self::MAX_LENGTH {
            return Err(format!(
                "too long: has {} but max allowed is {}",
                signature.len(),
                Self::MAX_LENGTH
            ));
        }

        let mut structure_stack = Vec::new();

        // TODO: check for max depth
        // TODO: check that array type code is followed by complete type; i.e. type definition
        // (including the stack) can't cross the boundary of an array. only go fully around it.
        // solve with recursion
        for (pos, c) in signature.chars().enumerate() {
            if Self::VALID_CHARS.contains(&c) {
                continue;
            }
            match c {
                '(' => structure_stack.push(c),
                ')' => {
                    let Some(last_opened) = structure_stack.pop() else {
                        return Err(format!("unopened {:?}", ')'));
                    };
                    match last_opened {
                        '(' => (),
                        '{' => {
                            return Err(format!(
                                "incorrect nesting: got {:?} (pos {}) but expected {:?}",
                                ')', pos, '}'
                            ));
                        }
                        _ => unreachable!(),
                    }
                }
                '{' => structure_stack.push(c),
                '}' => {
                    let Some(last_opened) = structure_stack.pop() else {
                        return Err(format!("unopened {:?}", '}'));
                    };
                    match last_opened {
                        '(' => {
                            return Err(format!(
                                "incorrect nesting: got {:?} (pos {}) but expected {:?}",
                                '}', pos, ')'
                            ));
                        }
                        '{' => (),
                        _ => unreachable!(),
                    }
                }
                invalid_char => return Err(format!("invalid type code: {:?}", invalid_char)),
            }
        }

        if !structure_stack.is_empty() {
            todo!("return error");
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        match str::from_utf8(&self.0) {
            Ok(s) => s,
            Err(_) => unreachable!("signature.0 is generated from valid String in constructor"),
        }
    }

    /// Returns the alignment of the value corresponding to this signature.
    pub fn get_value_alignment(&self) -> usize {
        match TypeCode::try_from(self.0[0]) {
            Ok(type_code) => type_code.get_alignment(),
            Err(_) => {
                unreachable!("validation during construction should ensure only valid type codes")
            }
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.0.len() == 1
    }

    pub fn read_value<T: ByteOrder>(
        &self,
        reader: &mut MessageReader<impl std::io::Read>,
    ) -> std::io::Result<Value> {
        if !self.is_primitive() {
            todo!();
        }
        Ok(match self.0[0] {
            b'y' => Value::Byte(reader.read::<T, u8>()?),
            b'b' => Value::Boolean(reader.read::<T, bool>()?),
            b'n' => Value::I16(reader.read::<T, i16>()?),
            b'q' => Value::U16(reader.read::<T, u16>()?),
            b'i' => Value::I32(reader.read::<T, i32>()?),
            b'u' => Value::U32(reader.read::<T, u32>()?),
            b'x' => Value::I64(reader.read::<T, i64>()?),
            b't' => Value::U64(reader.read::<T, u64>()?),
            b'd' => Value::Double(reader.read::<T, f64>()?),
            b's' => Value::String(reader.read::<T, String>()?),
            b'o' => Value::ObjectPath(reader.read::<T, ObjectPath>()?),
            b'g' => Value::Signature(reader.read::<T, Signature>()?),
            b'a' => unreachable!(),
            b'r' => unreachable!(),
            b'v' => Value::Variant(Box::new(reader.read::<T, Variant>()?)),
            b'e' => unreachable!(),
            b'h' => todo!(),
            _ => unreachable!(),
        })
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

impl From<TypeCode> for Signature {
    fn from(type_code: TypeCode) -> Self {
        match type_code {
            TypeCode::Byte => Signature::new_byte(),
            TypeCode::Boolean => Signature::new_boolean(),
            TypeCode::I16 => Signature::new_i16(),
            TypeCode::U16 => Signature::new_u16(),
            TypeCode::I32 => Signature::new_i32(),
            TypeCode::U32 => Signature::new_u32(),
            TypeCode::I64 => Signature::new_i64(),
            TypeCode::U64 => Signature::new_u64(),
            TypeCode::Double => Signature::new_double(),
            TypeCode::String => Signature::new_string(),
            TypeCode::ObjectPath => Signature::new_object_path(),
            TypeCode::Signature => Signature::new_signature(),
            TypeCode::Array => panic!("can't create signature for {:?}", type_code),
            TypeCode::Struct => panic!("can't create signature for {:?}", type_code),
            TypeCode::Variant => Signature::new_variant(),
            TypeCode::DictEntry => panic!("can't create signature for {:?}", type_code),
            TypeCode::UnixFd => Signature::new_unix_fd(),
        }
    }
}

impl TryFrom<String> for Signature {
    type Error = String;

    fn try_from(signature_string: String) -> Result<Self, Self::Error> {
        Self::validate(&signature_string).map(|_| Self(signature_string.into_bytes()))
    }
}

impl<'a> TryFrom<&'a str> for Signature {
    type Error = String;

    fn try_from(signature_string: &'a str) -> Result<Self, Self::Error> {
        Self::validate(signature_string).map(|_| Self(signature_string.to_owned().into_bytes()))
    }
}

impl std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Signature").field(&self.as_str()).finish()
    }
}
