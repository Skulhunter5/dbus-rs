use crate::wire_format::{WireFormatRead, WireFormatType, WireFormatWrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MajorProtocolVersion(u8);

impl WireFormatType for MajorProtocolVersion {
    const ALIGNMENT: usize = std::mem::size_of::<u8>();
}

impl WireFormatRead for MajorProtocolVersion {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        reader.read::<T, u8>().map(Self::from)
    }
}

impl WireFormatWrite for MajorProtocolVersion {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, u8>(self.0)
    }
}

impl From<u8> for MajorProtocolVersion {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MajorProtocolVersion> for u8 {
    fn from(value: MajorProtocolVersion) -> Self {
        value.0
    }
}
