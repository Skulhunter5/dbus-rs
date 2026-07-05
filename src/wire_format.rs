use std::io::{Read, Write};

use byteorder::ByteOrder;

mod message_reader;
mod message_writer;
pub use message_reader::MessageReader;
pub use message_writer::MessageWriter;

use crate::types::Signature;

trait StringLengthType: WireFormatType + WireFormatRead + WireFormatWrite {
    fn to_usize(&self) -> usize;
}

impl StringLengthType for u8 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl StringLengthType for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
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

// pub trait WireFormatType: Sized {
//     const ALIGNMENT: usize;
//
//     fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self>;
//     fn write_to<T: ByteOrder, W: Write>(
//         &self,
//         writer: &mut MessageWriter<W>,
//     ) -> std::io::Result<()>;
// }

pub trait WireFormatType: Sized {
    const ALIGNMENT: usize;
}

pub trait WireFormatRead: WireFormatType {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self>;
}

pub trait WireFormatWrite: WireFormatType {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()>;
}

impl<K: WireFormatType> WireFormatType for &K {
    const ALIGNMENT: usize = K::ALIGNMENT;
}

impl<K: WireFormatWrite> WireFormatWrite for &K {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        let k = *self;
        k.write_to::<T, _>(writer)
    }
}

impl WireFormatType for bool {
    // because booleans are transferred as u32
    const ALIGNMENT: usize = std::mem::size_of::<u32>();
}

impl WireFormatRead for bool {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_bool::<T>()
    }
}

impl WireFormatWrite for bool {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_bool::<T>(*self)
    }
}

impl WireFormatType for u8 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for u8 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u8()
    }
}

impl WireFormatWrite for u8 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u8(*self)
    }
}

impl WireFormatType for u16 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for u16 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u16::<T>()
    }
}

impl WireFormatWrite for u16 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u16::<T>(*self)
    }
}

impl WireFormatType for i16 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for i16 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_i16::<T>()
    }
}

impl WireFormatWrite for i16 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_i16::<T>(*self)
    }
}

impl WireFormatType for u32 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for u32 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u32::<T>()
    }
}

impl WireFormatWrite for u32 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u32::<T>(*self)
    }
}

impl WireFormatType for i32 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for i32 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_i32::<T>()
    }
}

impl WireFormatWrite for i32 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_i32::<T>(*self)
    }
}

impl WireFormatType for u64 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for u64 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u64::<T>()
    }
}

impl WireFormatWrite for u64 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u64::<T>(*self)
    }
}

impl WireFormatType for i64 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();
}

impl WireFormatRead for i64 {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_i64::<T>()
    }
}

impl WireFormatWrite for i64 {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_i64::<T>(*self)
    }
}

impl<E: WireFormatType> WireFormatType for Vec<E> {
    // because arrays start with a u32 for the length
    const ALIGNMENT: usize = std::mem::size_of::<u32>();
}

impl<E: WireFormatRead> WireFormatRead for Vec<E> {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_array::<T, E>()
    }
}

impl<E: WireFormatWrite> WireFormatWrite for Vec<E> {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_array::<T, E>(self)
    }
}

impl<E: WireFormatType> WireFormatType for &[E] {
    // because arrays start with a u32 for the length
    const ALIGNMENT: usize = std::mem::size_of::<u32>();
}

impl<E: WireFormatWrite> WireFormatWrite for &[E] {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_array::<T, E>(self)
    }
}

impl WireFormatType for String {
    // because strings start with a u32 for the length
    const ALIGNMENT: usize = std::mem::size_of::<u32>();
}

impl WireFormatRead for String {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_string::<T, u32>()
    }
}

impl WireFormatWrite for String {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_string::<T, u32>(self)
    }
}

impl WireFormatType for &str {
    // because strings start with a u32 for the length
    const ALIGNMENT: usize = std::mem::size_of::<u32>();
}

impl WireFormatWrite for &str {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_string::<T, u32>(self)
    }
}

impl WireFormatType for Signature {
    // because signatures start with a u8 for the length
    const ALIGNMENT: usize = std::mem::size_of::<u8>();
}

impl WireFormatRead for Signature {
    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        Self::try_from(reader.read_string::<T, u8>()?).map_err(|invalid_signature| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid signature {:?}", invalid_signature),
            )
        })
    }
}

impl WireFormatWrite for Signature {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_string::<T, u8>(self)
    }
}
