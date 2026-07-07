use std::io::{Read, Write};

use byteorder::ByteOrder;

use crate::{
    types::Signature,
    wire_format::{MessageReader, MessageWriter, WireFormatRead, WireFormatType, WireFormatWrite},
};

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
        Self::try_from(reader.read_string::<T, u8>()?).map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid signature {:?}", error),
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
