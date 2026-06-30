use crate::wire_format::WireFormatType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderField {
    code: u8,
}

impl WireFormatType for HeaderField {
    const ALIGNMENT: usize = 8;

    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        reader.align_to(Self::ALIGNMENT)?;
        todo!()
    }

    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.align_to(Self::ALIGNMENT)?;
        todo!();
    }
}
