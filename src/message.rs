use crate::Connection;

mod endianness;
mod flags;
mod message_type;
mod protocol_version;
pub use endianness::Endianness;
pub use flags::Flags;
pub use message_type::MessageType;
pub use protocol_version::MajorProtocolVersion;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    endianness: Endianness,
    ty: MessageType,
    flags: Flags,
    major_protocol_version: MajorProtocolVersion,
    serial: u32,
    fields: Vec<()>,
}

impl Message {
    pub(crate) fn read_from(connection: &mut Connection) -> std::io::Result<Self> {
        let endianness = Endianness::read_from(connection)?;
        let ty = MessageType::read_from(connection)?;
        let flags = Flags::read_from(connection)?;
        let protocol_version = MajorProtocolVersion::read_from(connection)?;

        match endianness {
            Endianness::LittleEndian => todo!(),
            Endianness::BigEndian => todo!(),
        }
    }
}
