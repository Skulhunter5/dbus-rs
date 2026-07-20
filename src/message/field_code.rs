use crate::wire_format::{WireFormatRead, WireFormatType, WireFormatWrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FieldCode {
    Path = 1,
    Interface = 2,
    Member = 3,
    ErrorName = 4,
    ReplySerial = 5,
    Destination = 6,
    Sender = 7,
    Signature = 8,
    UnixFds = 9,
}

impl WireFormatType for FieldCode {
    const ALIGNMENT: usize = std::mem::size_of::<u8>();
}

impl WireFormatRead for FieldCode {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        Self::try_from(reader.read::<T, u8>()?).map_err(|invalid_code| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid header field code: {}", invalid_code),
            )
        })
    }
}

impl WireFormatWrite for FieldCode {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, u8>(&(*self as u8))
    }
}

impl From<FieldCode> for u8 {
    fn from(value: FieldCode) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for FieldCode {
    type Error = u8;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        Ok(match code {
            1 => Self::Path,
            2 => Self::Interface,
            3 => Self::Member,
            4 => Self::ErrorName,
            5 => Self::ReplySerial,
            6 => Self::Destination,
            7 => Self::Sender,
            8 => Self::Signature,
            9 => Self::UnixFds,
            invalid_code => return Err(invalid_code),
        })
    }
}
