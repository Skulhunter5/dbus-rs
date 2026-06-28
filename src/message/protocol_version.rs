use crate::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MajorProtocolVersion(u8);

impl MajorProtocolVersion {
    pub(crate) fn read_from(connection: &mut Connection) -> std::io::Result<Self> {
        connection.read_u8().map(|byte| Self::from(byte))
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
