use crate::message::{MessageReader, MessageWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    Invalid = 0,
    MethodCall = 1,
    MethodReply = 2,
    ErrorReply = 3,
    Signal = 4,
}

impl MessageType {
    pub(crate) fn read_from(
        reader: &mut MessageReader<impl std::io::Read>,
    ) -> std::io::Result<Self> {
        Self::try_from(reader.read_u8()?).map_err(|invalid_byte| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid header message type byte: {}", invalid_byte),
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
