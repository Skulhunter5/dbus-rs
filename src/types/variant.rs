use crate::{
    types::{Signature, Value},
    wire_format::{WireFormatRead, WireFormatType, WireFormatWrite},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variant {
    signature: Signature,
    value: Value,
}

impl Variant {
    pub fn get_signature(&self) -> &Signature {
        &self.signature
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}

impl From<Variant> for Value {
    fn from(variant: Variant) -> Self {
        variant.value
    }
}

// TODO: make sure that this alignment is correct; i.e. that it's not supposed to be a struct and
// therefore aligned like a struct at 8 bytes
impl WireFormatType for Variant {
    const ALIGNMENT: usize = Signature::ALIGNMENT;
}

impl WireFormatRead for Variant {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        let signature = reader.read::<T, Signature>()?;
        let value = signature.read_value::<T>(reader)?;
        Ok(Self { signature, value })
    }
}

impl WireFormatWrite for Variant {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, _>(&self.signature)?;
        self.value.write_to::<T>(writer)?;
        Ok(())
    }
}
