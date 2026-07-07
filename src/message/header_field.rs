use crate::{
    message::FieldCode,
    types::{ObjectPath, Signature, TypeCode, Value, Variant},
    wire_format::{WireFormatRead, WireFormatType, WireFormatWrite},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeaderField {
    Path(ObjectPath),
    Interface(String),
    Member(String),
    ErrorName(String),
    ReplySerial(u32),
    Destination(String),
    Sender(String),
    Signature(Signature),
    UnixFds(u32),
}

impl HeaderField {
    fn field_code(&self) -> FieldCode {
        match self {
            Self::Path(_) => FieldCode::Path,
            Self::Interface(_) => FieldCode::Interface,
            Self::Member(_) => FieldCode::Member,
            Self::ErrorName(_) => FieldCode::ErrorName,
            Self::ReplySerial(_) => FieldCode::ReplySerial,
            Self::Destination(_) => FieldCode::Destination,
            Self::Sender(_) => FieldCode::Sender,
            Self::Signature(_) => FieldCode::Signature,
            Self::UnixFds(_) => FieldCode::UnixFds,
        }
    }

    fn signature(&self) -> Signature {
        match self {
            Self::Path(_) => Signature::new_object_path(),
            Self::Interface(_) => Signature::new_string(),
            Self::Member(_) => Signature::new_string(),
            Self::ErrorName(_) => Signature::new_string(),
            Self::ReplySerial(_) => Signature::new_u32(),
            Self::Destination(_) => Signature::new_string(),
            Self::Sender(_) => Signature::new_string(),
            Self::Signature(_) => Signature::new_signature(),
            Self::UnixFds(_) => Signature::new_u32(),
        }
    }
}

impl WireFormatType for HeaderField {
    // struct alignment
    const ALIGNMENT: usize = 8;
}

impl WireFormatRead for HeaderField {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        reader.align_to(Self::ALIGNMENT)?;
        let field_code = reader.read::<T, FieldCode>()?;
        let variant = reader.read::<T, Variant>()?;
        let value = variant.into_value();
        match field_code {
            FieldCode::Path => match value {
                Value::ObjectPath(path) => Ok(Self::Path(path)),
                incorrect_value => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "incorrect value for header field {:?}: got {:?} but expected {:?}",
                        field_code,
                        incorrect_value,
                        TypeCode::ObjectPath
                    ),
                )),
            },
            FieldCode::Interface => match value {
                // TODO: validate as interface name? or is that the responsibility of the downstream
                // consumer of this value?
                Value::String(interface) => Ok(Self::Interface(interface)),
                incorrect_value => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "incorrect value for header field {:?}: got {:?} but expected {:?}",
                        field_code,
                        incorrect_value,
                        TypeCode::String
                    ),
                )),
            },
            FieldCode::Member => match value {
                // TODO: validate as member name or signal name? or is that the responsibility of
                // the downstream consumer of this value?
                Value::String(member) => Ok(Self::Member(member)),
                incorrect_value => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "incorrect value for header field {:?}: got {:?} but expected {:?}",
                        field_code,
                        incorrect_value,
                        TypeCode::String
                    ),
                )),
            },
            FieldCode::ErrorName => todo!(),
            FieldCode::ReplySerial => todo!(),
            FieldCode::Destination => match value {
                Value::String(destination) => Ok(Self::Destination(destination)),
                incorrect_value => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "incorrect value for header field {:?}: got {:?} but expected {:?}",
                        field_code,
                        incorrect_value,
                        TypeCode::String
                    ),
                )),
            },
            FieldCode::Sender => todo!(),
            FieldCode::Signature => todo!(),
            FieldCode::UnixFds => todo!(),
        }
    }
}

impl WireFormatWrite for HeaderField {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        // header field has additional alignment restriction because it's a struct
        writer.align_to(Self::ALIGNMENT)?;
        writer.write::<T, FieldCode>(self.field_code())?;
        writer.write::<T, Signature>(self.signature())?;
        match self {
            Self::Path(value) => writer.write::<T, &ObjectPath>(value)?,
            Self::Interface(value) => writer.write::<T, &str>(value)?,
            Self::Member(value) => writer.write::<T, &str>(value)?,
            Self::ErrorName(value) => writer.write::<T, &str>(value)?,
            Self::ReplySerial(value) => writer.write::<T, &u32>(value)?,
            Self::Destination(value) => writer.write::<T, &str>(value)?,
            Self::Sender(value) => writer.write::<T, &str>(value)?,
            Self::Signature(value) => writer.write::<T, &Signature>(value)?,
            Self::UnixFds(value) => writer.write::<T, &u32>(value)?,
        }
        Ok(())
    }
}
