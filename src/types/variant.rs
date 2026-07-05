use crate::{
    types::Signature,
    wire_format::{WireFormatRead, WireFormatType, WireFormatWrite},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variant {
    signature: Signature,
    value: Vec<u8>,
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
        let value = signature.read_value_bytes::<T>(reader)?;
        todo!()
    }
}

impl WireFormatWrite for Variant {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, _>(&self.signature)?;
        todo!();
    }
}
