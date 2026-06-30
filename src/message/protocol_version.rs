use crate::message::{MessageReader, MessageWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MajorProtocolVersion(u8);

impl MajorProtocolVersion {
    pub(crate) fn read_from(
        reader: &mut MessageReader<impl std::io::Read>,
    ) -> std::io::Result<Self> {
        reader.read_u8().map(|byte| Self::from(byte))
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut MessageWriter<impl std::io::Write>,
    ) -> std::io::Result<()> {
        writer.write_u8(self.0)
    }
}

impl From<u8> for MajorProtocolVersion {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for MajorProtocolVersion {
    fn into(self) -> u8 {
        self.0
    }
}
