use crate::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Endianness {
    LittleEndian = b'l',
    BigEndian = b'B',
}

impl Endianness {
    pub(crate) fn read_from(connection: &mut Connection) -> std::io::Result<Self> {
        Self::try_from(connection.read_u8()?).map_err(|invalid_byte| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid header endianness byte: {}", invalid_byte),
            )
        })
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
