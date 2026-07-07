use std::io::{Read, Write};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

mod endianness;
mod field_code;
mod flags;
mod header_field;
mod message_type;
mod protocol_version;
pub use endianness::Endianness;
pub use field_code::FieldCode;
pub use flags::Flags;
pub use header_field::HeaderField;
pub use message_type::MessageType;
pub use protocol_version::MajorProtocolVersion;

use crate::wire_format::{MessageReader, MessageWriter};

// TODO: add proper validation for header_fields, ensuring they are correct for the MessageType

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    pub endianness: Endianness,
    pub ty: MessageType,
    pub flags: Flags,
    pub major_protocol_version: MajorProtocolVersion,
    pub serial: u32,
    pub header_fields: Vec<HeaderField>,
    pub body: Vec<u8>,
}

impl Message {
    pub fn read_from(mut reader: MessageReader<impl Read>) -> std::io::Result<Self> {
        let endianness = reader.read::<LittleEndian, Endianness>()?;
        let ty = reader.read::<LittleEndian, MessageType>()?;
        let flags = reader.read::<LittleEndian, Flags>()?;
        let major_protocol_version = reader.read::<LittleEndian, MajorProtocolVersion>()?;

        fn inner_read<T: ByteOrder>(
            reader: &mut MessageReader<impl Read>,
        ) -> std::io::Result<(u32, u32, Vec<HeaderField>)> {
            let length = reader.read::<T, u32>()?;
            let serial = reader.read::<T, u32>()?;
            let header_fields = reader.read::<T, Vec<HeaderField>>()?;

            Ok((length, serial, header_fields))
        }

        let (length, serial, header_fields) = match endianness {
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
            header_fields,
            body,
        })
    }

    pub fn write_to(&self, mut writer: MessageWriter<impl Write>) -> std::io::Result<()> {
        writer.write::<LittleEndian, _>(self.endianness)?;
        writer.write::<LittleEndian, _>(self.ty)?;
        writer.write::<LittleEndian, _>(self.flags)?;
        writer.write::<LittleEndian, _>(self.major_protocol_version)?;

        fn inner_write<T: ByteOrder>(
            writer: &mut MessageWriter<impl Write>,
            length: u32,
            serial: u32,
            header_fields: &[HeaderField],
        ) -> std::io::Result<()> {
            writer.write::<T, _>(length)?;
            writer.write::<T, _>(serial)?;
            writer.write_array::<T, _>(header_fields)?;

            Ok(())
        }

        match self.endianness {
            Endianness::LittleEndian => inner_write::<LittleEndian>(
                &mut writer,
                self.body.len() as u32,
                self.serial,
                &self.header_fields,
            )?,
            Endianness::BigEndian => inner_write::<BigEndian>(
                &mut writer,
                self.body.len() as u32,
                self.serial,
                &self.header_fields,
            )?,
        }

        writer.write_body(&self.body)
    }
}
