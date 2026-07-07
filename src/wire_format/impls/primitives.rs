use std::io::{Read, Write};

use byteorder::ByteOrder;

use crate::wire_format::{
    MessageReader, MessageWriter, WireFormatRead, WireFormatType, WireFormatWrite,
};

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
