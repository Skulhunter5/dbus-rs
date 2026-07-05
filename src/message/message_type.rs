use crate::wire_format::{WireFormatRead, WireFormatType, WireFormatWrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    Invalid = 0,
    MethodCall = 1,
    MethodReply = 2,
    ErrorReply = 3,
    Signal = 4,
}

impl WireFormatType for MessageType {
    const ALIGNMENT: usize = std::mem::size_of::<u8>();
}

impl WireFormatRead for MessageType {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        Self::try_from(reader.read::<T, u8>()?).map_err(|invalid_byte| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid header message type byte: {}", invalid_byte),
            )
        })
    }
}

impl WireFormatWrite for MessageType {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, u8>(*self as u8)
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for MessageType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Invalid,
            1 => Self::MethodCall,
            2 => Self::MethodReply,
            3 => Self::ErrorReply,
            4 => Self::Signal,
            invalid_value => return Err(invalid_value),
        })
    }
}
