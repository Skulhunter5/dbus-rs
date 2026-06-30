use std::io::{Read, Write};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

mod endianness;
mod flags;
mod message_type;
mod protocol_version;
mod reader;
mod writer;
pub use endianness::Endianness;
pub use flags::Flags;
pub use message_type::MessageType;
pub use protocol_version::MajorProtocolVersion;
pub(crate) use reader::MessageReader;
pub(crate) use writer::MessageWriter;

use crate::wire_format::WireFormatType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderField {
    code: u8,
}

impl WireFormatType for HeaderField {
    const ALIGNMENT: usize = 8;

    fn read_from<T: ByteOrder, R: std::io::Read>(
        reader: &mut MessageReader<R>,
    ) -> std::io::Result<Self> {
        todo!()
    }

    fn write_to<T: ByteOrder, W: Write>(
        &self,
        writer: &mut MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.align_to(Self::ALIGNMENT)?;
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    endianness: Endianness,
    ty: MessageType,
    flags: Flags,
    major_protocol_version: MajorProtocolVersion,
    serial: u32,
    fields: Vec<HeaderField>,
    body: Vec<u8>,
}

impl Message {
    pub(crate) fn read_from(mut reader: MessageReader<impl Read>) -> std::io::Result<Self> {
        let endianness = Endianness::read_from(&mut reader)?;
        let ty = MessageType::read_from(&mut reader)?;
        let flags = Flags::read_from(&mut reader)?;
        let major_protocol_version = MajorProtocolVersion::read_from(&mut reader)?;

        fn inner_read<T: ByteOrder>(
            reader: &mut MessageReader<impl Read>,
        ) -> std::io::Result<(u32, u32, Vec<HeaderField>)> {
            let length = reader.read_u32::<T>()?;
            let serial = reader.read_u32::<T>()?;
            let fields = reader.read_array::<T, HeaderField>()?;

            Ok((length, serial, fields))
        }

        let (length, serial, fields) = match endianness {
            Endianness::LittleEndian => inner_read::<LittleEndian>(&mut reader)?,
            Endianness::BigEndian => inner_read::<BigEndian>(&mut reader)?,
        };

        let body = reader.read_body(length as usize)?;

        Ok(Self {
            endianness,
            ty,
            flags,
            major_protocol_version,
            serial,
            fields,
            body,
        })
    }

    pub(crate) fn write_to(&self, mut writer: MessageWriter<impl Write>) -> std::io::Result<()> {
        self.endianness.write_to(&mut writer)?;
        self.ty.write_to(&mut writer)?;
        self.flags.write_to(&mut writer)?;
        self.major_protocol_version.write_to(&mut writer)?;

        fn inner_write<T: ByteOrder>(
            writer: &mut MessageWriter<impl Write>,
            length: u32,
            serial: u32,
            fields: &Vec<HeaderField>,
        ) -> std::io::Result<()> {
            writer.write_u32::<T>(length)?;
            writer.write_u32::<T>(serial)?;
            writer.write_array::<T, _>(&fields)?;

            Ok(())
        }

        match self.endianness {
            Endianness::LittleEndian => inner_write::<LittleEndian>(
                &mut writer,
                self.body.len() as u32,
                self.serial,
                &self.fields,
            )?,
            Endianness::BigEndian => inner_write::<BigEndian>(
                &mut writer,
                self.body.len() as u32,
                self.serial,
                &self.fields,
            )?,
        }

        writer.write_body(&self.body)
    }
}
