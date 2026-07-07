use byteorder::ByteOrder;

use crate::{
    types::{ObjectPath, Signature, Variant},
    wire_format::MessageWriter,
};

// TODO: Add variants for names (BusName, InterfaceName, MemberName, ErrorName) etc.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Byte(u8),
    Boolean(bool),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Double(f64),
    String(String),
    ObjectPath(ObjectPath),
    Signature(Signature),
    Variant(Box<Variant>),
}

impl Value {
    pub fn write_to<T: ByteOrder>(
        &self,
        writer: &mut MessageWriter<impl std::io::Write>,
    ) -> std::io::Result<()> {
        match self {
            Self::Byte(value) => writer.write::<T, &u8>(value),
            Self::Boolean(value) => writer.write::<T, &bool>(value),
            Self::I16(value) => writer.write::<T, &i16>(value),
            Self::U16(value) => writer.write::<T, &u16>(value),
            Self::I32(value) => writer.write::<T, &i32>(value),
            Self::U32(value) => writer.write::<T, &u32>(value),
            Self::I64(value) => writer.write::<T, &i64>(value),
            Self::U64(value) => writer.write::<T, &u64>(value),
            Self::Double(value) => writer.write::<T, &f64>(value),
            Self::String(value) => writer.write::<T, &String>(value),
            Self::ObjectPath(value) => writer.write::<T, &ObjectPath>(value),
            Self::Signature(value) => writer.write::<T, &Signature>(value),
            Self::Variant(value) => writer.write::<T, &Variant>(value),
        }
    }
}
