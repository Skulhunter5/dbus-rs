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
    Struct(Box<[Value]>),
}

impl Value {
    pub fn write_to<T: ByteOrder>(
        &self,
        writer: &mut MessageWriter<impl std::io::Write>,
    ) -> std::io::Result<()> {
        match self {
            Self::Byte(value) => writer.write::<T, u8>(value),
            Self::Boolean(value) => writer.write::<T, bool>(value),
            Self::I16(value) => writer.write::<T, i16>(value),
            Self::U16(value) => writer.write::<T, u16>(value),
            Self::I32(value) => writer.write::<T, i32>(value),
            Self::U32(value) => writer.write::<T, u32>(value),
            Self::I64(value) => writer.write::<T, i64>(value),
            Self::U64(value) => writer.write::<T, u64>(value),
            Self::Double(value) => writer.write::<T, f64>(value),
            Self::String(value) => writer.write::<T, String>(value),
            Self::ObjectPath(value) => writer.write::<T, ObjectPath>(value),
            Self::Signature(value) => writer.write::<T, Signature>(value),
            Self::Variant(value) => writer.write::<T, Variant>(value),
            Self::Struct(value) => writer.write_struct::<T>(value),
        }
    }

    pub fn borrow(&self) -> BorrowedValue<'_> {
        match self {
            Self::Byte(value) => BorrowedValue::Byte(value),
            Self::Boolean(value) => BorrowedValue::Boolean(value),
            Self::I16(value) => BorrowedValue::I16(value),
            Self::U16(value) => BorrowedValue::U16(value),
            Self::I32(value) => BorrowedValue::I32(value),
            Self::U32(value) => BorrowedValue::U32(value),
            Self::I64(value) => BorrowedValue::I64(value),
            Self::U64(value) => BorrowedValue::U64(value),
            Self::Double(value) => BorrowedValue::Double(value),
            Self::String(value) => BorrowedValue::String(value),
            Self::ObjectPath(value) => BorrowedValue::ObjectPath(value),
            Self::Signature(value) => BorrowedValue::Signature(value),
            Self::Variant(value) => BorrowedValue::Variant(value),
            Self::Struct(field_values) => BorrowedValue::Struct(
                field_values
                    .iter()
                    .map(|value| value.borrow())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BorrowedValue<'a> {
    Byte(&'a u8),
    Boolean(&'a bool),
    I16(&'a i16),
    U16(&'a u16),
    I32(&'a i32),
    U32(&'a u32),
    I64(&'a i64),
    U64(&'a u64),
    Double(&'a f64),
    String(&'a str),
    ObjectPath(&'a ObjectPath),
    Signature(&'a Signature),
    Variant(&'a Variant),
    Struct(Box<[BorrowedValue<'a>]>),
}

impl<'a> BorrowedValue<'a> {
    pub fn write_to<T: ByteOrder>(
        &self,
        writer: &mut MessageWriter<impl std::io::Write>,
    ) -> std::io::Result<()> {
        match self {
            Self::Byte(value) => writer.write::<T, u8>(value),
            Self::Boolean(value) => writer.write::<T, bool>(value),
            Self::I16(value) => writer.write::<T, i16>(value),
            Self::U16(value) => writer.write::<T, u16>(value),
            Self::I32(value) => writer.write::<T, i32>(value),
            Self::U32(value) => writer.write::<T, u32>(value),
            Self::I64(value) => writer.write::<T, i64>(value),
            Self::U64(value) => writer.write::<T, u64>(value),
            Self::Double(value) => writer.write::<T, f64>(value),
            Self::String(value) => writer.write::<T, str>(value),
            Self::ObjectPath(value) => writer.write::<T, ObjectPath>(value),
            Self::Signature(value) => writer.write::<T, Signature>(value),
            Self::Variant(value) => writer.write::<T, Variant>(value),
            Self::Struct(value) => writer.write_borrowed_struct::<T>(value),
        }
    }
}
