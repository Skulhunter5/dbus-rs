use crate::message::{MessageReader, MessageWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flags(u8);

impl Flags {
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

    pub fn none() -> Self {
        Self(0)
    }

    pub fn no_reply_expected(&self) -> bool {
        self.0 & 0x1 != 0
    }

    pub fn no_auto_start(&self) -> bool {
        self.0 & 0x2 != 0
    }

    pub fn allow_interactive_authorization(&self) -> bool {
        self.0 & 0x4 != 0
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        self.0
    }
}
