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
            let body_bytes = reader.read_body(length as usize)?;

            let signature = header_fields.iter().find_map(|field| match field {
                HeaderField::Signature(signature) => Some(signature),
                _ => None,
            });
            if let Some(signature) = signature {
                let mut body_bytes = &body_bytes[..];
                let mut body_reader = MessageReader::new_buffer_reader(&mut body_bytes);

                let body_value = signature.read_value_from::<T>(&mut body_reader).map_err(
                    |error| match error {
                        x if x.kind() == std::io::ErrorKind::UnexpectedEof => std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "exceeded body length when reading the body",
                        ),
                        x => x,
                    },
                )?;

                Ok((serial, header_fields, Some(body_value)))
            } else {
                Ok((serial, header_fields, None))
            }
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

        fn inner_write<T: ByteOrder>(
            mut writer: MessageWriter<impl Write>,
            serial: u32,
            header_fields: &[HeaderField],
            body: Option<&Value>,
        ) -> std::io::Result<()> {
            let mut buffer = Vec::new();
            if let Some(body) = body {
                let mut buffer_writer = MessageWriter::new_buffer_writer(&mut buffer);
                body.write_to::<T>(&mut buffer_writer)?;
            }
            let body_bytes = &buffer;

            writer.write::<T, u32>(&(body_bytes.len() as u32))?;
            writer.write::<T, u32>(&serial)?;
            writer.write::<T, [HeaderField]>(header_fields)?;

            writer.write_body(body_bytes)
        }

        match self.endianness {
            Endianness::LittleEndian => inner_write::<LittleEndian>(
                writer,
                self.serial,
                &self.header_fields,
                self.body.as_ref(),
            ),
            Endianness::BigEndian => inner_write::<BigEndian>(
                writer,
                self.serial,
                &self.header_fields,
                self.body.as_ref(),
            ),
        }
    }
}
