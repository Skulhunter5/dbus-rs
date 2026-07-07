use crate::{types::Signature, wire_format::WireFormatType};

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

impl TypeCode {
    pub fn get_alignment(&self) -> usize {
        match self {
            Self::Byte => <u8 as WireFormatType>::ALIGNMENT,
            Self::Boolean => <bool as WireFormatType>::ALIGNMENT,
            Self::I16 => <i16 as WireFormatType>::ALIGNMENT,
            Self::U16 => <u16 as WireFormatType>::ALIGNMENT,
            Self::I32 => <i32 as WireFormatType>::ALIGNMENT,
            Self::U32 => <u32 as WireFormatType>::ALIGNMENT,
            Self::I64 => <i64 as WireFormatType>::ALIGNMENT,
            Self::U64 => <u64 as WireFormatType>::ALIGNMENT,
            Self::Double => <f64 as WireFormatType>::ALIGNMENT,
            Self::String => <String as WireFormatType>::ALIGNMENT,
            Self::ObjectPath => todo!(),
            Self::Signature => <Signature as WireFormatType>::ALIGNMENT,
            // Arrays always have the same alignment due to the u32 size prefix, so the inner type
            // doesn't matter
            Self::Array => <Vec<u8> as WireFormatType>::ALIGNMENT,
            Self::Struct => todo!(),
            Self::Variant => todo!(),
            Self::DictEntry => todo!(),
            Self::UnixFd => todo!(),
        }
    }
}

impl TryFrom<char> for TypeCode {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let code: u8 = match c.try_into() {
            Ok(code) => code,
            Err(_) => return Err(c),
        };
        Self::try_from(code).map_err(char::from)
    }
}

impl TryFrom<u8> for TypeCode {
    type Error = u8;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        Ok(match code {
            b'y' => Self::Byte,
            b'b' => Self::Boolean,
            b'n' => Self::I16,
            b'q' => Self::U16,
            b'i' => Self::I32,
            b'u' => Self::U32,
            b'x' => Self::I64,
            b't' => Self::U64,
            b'd' => Self::Double,
            b's' => Self::String,
            b'o' => Self::ObjectPath,
            b'g' => Self::Signature,
            b'a' => Self::Array,
            b'r' => Self::Struct,
            b'v' => Self::Variant,
            b'e' => Self::DictEntry,
            b'h' => Self::UnixFd,
            invalid_code => return Err(invalid_code),
        })
    }
}
