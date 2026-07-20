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

use crate::{
    types::Value,
    wire_format::{MessageReader, MessageWriter},
};

// TODO: add proper validation for header_fields, ensuring they are correct for the MessageType and
// that each header field only appears once

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Message {
    pub endianness: Endianness,
    pub ty: MessageType,
    pub flags: Flags,
    pub major_protocol_version: MajorProtocolVersion,
    pub serial: u32,
    pub header_fields: Vec<HeaderField>,
    pub body: Option<Value>,
}

impl Message {
    pub fn read_from(mut reader: MessageReader<impl Read>) -> std::io::Result<Self> {
        let endianness = reader.read::<LittleEndian, Endianness>()?;
        let ty = reader.read::<LittleEndian, MessageType>()?;
        let flags = reader.read::<LittleEndian, Flags>()?;
        let major_protocol_version = reader.read::<LittleEndian, MajorProtocolVersion>()?;

        fn inner_read<T: ByteOrder>(
            mut reader: MessageReader<impl Read>,
        ) -> std::io::Result<(u32, Vec<HeaderField>, Option<Value>)> {
            let length = reader.read::<T, u32>()?;
            let serial = reader.read::<T, u32>()?;
            let header_fields = reader.read::<T, Vec<HeaderField>>()?;

            let signature = header_fields.iter().find_map(|field| match field {
                HeaderField::Signature(signature) => Some(signature),
                _ => None,
            });
            let body = if let Some(signature) = signature {
                Some(reader.read_body::<T>(signature, length as usize)?)
            } else {
                None
            };

            Ok((serial, header_fields, body))
        }

        let (serial, header_fields, body) = match endianness {
            Endianness::LittleEndian => inner_read::<LittleEndian>(reader)?,
            Endianness::BigEndian => inner_read::<BigEndian>(reader)?,
        };

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
        writer.write::<LittleEndian, _>(&self.endianness)?;
        writer.write::<LittleEndian, _>(&self.ty)?;
        writer.write::<LittleEndian, _>(&self.flags)?;
        writer.write::<LittleEndian, _>(&self.major_protocol_version)?;

        match self.endianness {
            Endianness::LittleEndian => writer.write_message::<LittleEndian>(
                self.serial,
                &self.header_fields,
                self.body.as_ref(),
            ),
            Endianness::BigEndian => writer.write_message::<BigEndian>(
                self.serial,
                &self.header_fields,
                self.body.as_ref(),
            ),
        }
    }
}
