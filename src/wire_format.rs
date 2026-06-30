use std::io::{Read, Write};

use byteorder::ByteOrder;

mod message_reader;
mod message_writer;
pub use message_reader::MessageReader;
pub use message_writer::MessageWriter;

// TODO: split WireFormatType into read and write halves
// this would allow to have compile-time safety for reading and writing slices instead of the
// current panic!() in read

// TODO: figure out a way to get rid of <T: ByteOrder> for single-byte types like u8 and everything
// based on u8, like message::{MajorProtocolVersion, Flags, Endianness, MessageType}

// TODO: figure out if it's possible to get rid of the user having to always provide the generic
// type parameter for std::io::Read themselves (even though they can just use do <T, _>, it still is
// pretty annoying

// TODO: maybe move all of the actual implementations from reader/writer up into here
// then all of the `align_to` calls can use the same `const ALIGNMENT` that is defined up here,
// instead of having to define redefine the alignment themselves

pub trait WireFormatType: Sized {
    const ALIGNMENT: usize;

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self>;
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()>;
}

impl WireFormatType for bool {
    // because booleans are transferred as u32
    const ALIGNMENT: usize = std::mem::size_of::<u32>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_bool::<T>()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_bool::<T>(*self)
    }
}

impl WireFormatType for u8 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u8()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u8(*self)
    }
}

impl WireFormatType for u16 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u16::<T>()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u16::<T>(*self)
    }
}

impl WireFormatType for i16 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_i16::<T>()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_i16::<T>(*self)
    }
}

impl WireFormatType for u32 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u32::<T>()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u32::<T>(*self)
    }
}

impl WireFormatType for i32 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_i32::<T>()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_i32::<T>(*self)
    }
}

impl WireFormatType for u64 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u64::<T>()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_u64::<T>(*self)
    }
}

impl WireFormatType for i64 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_i64::<T>()
    }

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

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_array::<T, E>()
    }

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

    fn read_from<T: ByteOrder, R: Read>(_reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        panic!("can't read array as slice because ownership can't be transferred to the caller");
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_array::<T, E>(self)
    }
}
