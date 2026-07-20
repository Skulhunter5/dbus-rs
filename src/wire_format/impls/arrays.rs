use std::io::{Read, Write};

use byteorder::ByteOrder;

use crate::wire_format::{
    MessageReader, MessageWriter, WireFormatRead, WireFormatType, WireFormatWrite,
};

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

impl<E: WireFormatType> WireFormatType for [E] {
    // because arrays start with a u32 for the length
    const ALIGNMENT: usize = std::mem::size_of::<u32>();
}

impl<E: WireFormatWrite> WireFormatWrite for [E] {
    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write_array::<T, E>(self)
    }
}
