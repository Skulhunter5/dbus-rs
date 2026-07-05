use crate::{
    types::Variant,
    wire_format::{WireFormatRead, WireFormatType, WireFormatWrite},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderField {
    code: u8,
    variant: Variant,
}

impl WireFormatType for HeaderField {
    // struct alignment
    const ALIGNMENT: usize = 8;
}

impl WireFormatRead for HeaderField {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        reader.align_to(Self::ALIGNMENT)?;
        let code = reader.read::<T, u8>()?;
        let variant = reader.read::<T, Variant>()?;
        Ok(Self { code, variant })
    }
}

impl WireFormatWrite for HeaderField {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.align_to(Self::ALIGNMENT)?;
        writer.write::<T, _>(self.code)?;
        writer.write::<T, _>(&self.variant)?;
        Ok(())
    }
}
