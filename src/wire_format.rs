use std::io::{Read, Write};

use byteorder::ByteOrder;

mod impls;
mod message_reader;
mod message_writer;
pub use message_reader::MessageReader;
pub use message_writer::MessageWriter;

use crate::types::{BorrowedValue, Signature, Value};

trait StringLengthType: WireFormatType + WireFormatRead + WireFormatWrite {
    fn to_usize(&self) -> usize;
    fn from_usize(value: usize) -> Self;
}

impl StringLengthType for u8 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}

impl StringLengthType for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}

// TODO: figure out a way to get rid of <T: ByteOrder> for single-byte types like u8 and everything
//   based on u8, like message::{MajorProtocolVersion, Flags, Endianness, MessageType}

// TODO: figure out if it's possible to get rid of the user having to always provide the generic
//   type parameter for std::io::Read themselves (even though they can just use do <T, _>, it still is
// pretty annoying

// TODO: maybe move all of the actual implementations from reader/writer up into here
//   then all of the `align_to` calls can use the same `const ALIGNMENT` that is defined up here,
// instead of having to define redefine the alignment themselves
//   a problem with this is the requirement for offset to stay correct. that burden would then be on
// each type individually, which is quite annoying. A solution for this would be adding a function
// like `read_raw_bytes::<CONST N: usize, T: WireFormatType>(...) -> [u8; N]` that returns the raw bytes
// for a type, automatically aligning to `T::ALIGNMENT` and also automatically incrementing offset. Then
// the functions here would have to interpret the bytes correctly, including the ByteOrder. For
// this, the byteorder crate could probably be removed and everything be done by manually calling
// `from_le_bytes` and `from_be_bytes` respectively

pub trait WireFormatType {
    const ALIGNMENT: usize;

    fn get_signature() -> &'static Signature {
        todo!();
    }
}

pub trait WireFormatRead: WireFormatType + Sized {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self>;
}

pub trait WireFormatWrite: WireFormatType {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()>;
}

pub trait WireFormatStruct: Sized {
    fn get_signature() -> &'static Signature;

    fn construct(values: Box<[Value]>) -> std::io::Result<Self>;
    fn deconstruct<'a>(&'a self) -> Box<[BorrowedValue<'a>]>;
}

impl<S: WireFormatStruct> WireFormatType for S {
    // structs are always aligned to the full 8-byte boundary
    const ALIGNMENT: usize = 8;

    fn get_signature() -> &'static Signature {
        <Self as WireFormatStruct>::get_signature()
    }
}

impl<S: WireFormatStruct> WireFormatRead for S {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        let value = Self::get_signature().read_value_from::<T>(reader)?;
        let Value::Struct(values) = value else {
            panic!("struct type doesn't have a struct signature");
        };
        Self::construct(values)
    }
}

impl<S: WireFormatStruct> WireFormatWrite for S {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        let value = BorrowedValue::Struct(self.deconstruct());
        value.write_to::<T>(writer)
    }
}
