use std::io::{Read, Write};

use byteorder::ByteOrder;

use crate::message::{MessageReader, MessageWriter};

pub trait WireFormatType: Sized {
    const ALIGNMENT: usize;

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self>;
    fn write_to<T: ByteOrder, W: Write>(&self, writer: &mut MessageWriter<W>) -> std::io::Result<()>;
}

impl WireFormatType for u8 {
    const ALIGNMENT: usize = std::mem::size_of::<Self>();

    fn read_from<T: ByteOrder, R: Read>(reader: &mut MessageReader<R>) -> std::io::Result<Self> {
        reader.read_u8()
    }

    fn write_to<T: ByteOrder, W: Write>(&self, writer: &mut MessageWriter<W>) -> std::io::Result<()> {
        writer.write_u8(*self)
    }
}
