use crate::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flags(u8);

impl Flags {
    pub(crate) fn read_from(connection: &mut Connection) -> std::io::Result<Self> {
        connection.read_u8().map(|byte| Self::from(byte))
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
