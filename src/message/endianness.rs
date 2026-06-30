use crate::wire_format::WireFormatType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Endianness {
    LittleEndian = b'l',
    BigEndian = b'B',
}

impl WireFormatType for Endianness {
    const ALIGNMENT: usize = std::mem::size_of::<u8>();

    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        Self::try_from(reader.read::<T, u8>()?).map_err(|invalid_byte| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid header endianness byte: {}", invalid_byte),
            )
        })
    }

    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, u8>(*self as u8)
    }
}

impl From<Endianness> for u8 {
    fn from(value: Endianness) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Endianness {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'l' => Self::LittleEndian,
            b'B' => Self::BigEndian,
            invalid_value => return Err(invalid_value),
        })
    }
}
