use crate::message::{MessageReader, MessageWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Endianness {
    LittleEndian = b'l',
    BigEndian = b'B',
}

impl Endianness {
    pub(crate) fn read_from(
        reader: &mut MessageReader<impl std::io::Read>,
    ) -> std::io::Result<Self> {
        Self::try_from(reader.read_u8()?).map_err(|invalid_byte| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid header endianness byte: {}", invalid_byte),
            )
        })
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut MessageWriter<impl std::io::Write>,
    ) -> std::io::Result<()> {
        writer.write_u8(*self as u8)
    }
}

impl Into<u8> for Endianness {
    fn into(self) -> u8 {
        self as u8
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
