use crate::wire_format::{WireFormatRead, WireFormatType, WireFormatWrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flags(u8);

impl Flags {
    pub fn none() -> Self {
        Self(0)
    }

    const INDEX_NRE: u8 = 0;
    const INDEX_NAS: u8 = 1;
    const INDEX_AIA: u8 = 2;

    fn with(&self, index: u8, value: bool) -> Self {
        let mask = !(1 << index);
        Self((self.0 & mask) | ((value as u8) << index))
    }

    pub fn with_no_reply_expected(&self, no_reply_expected: bool) -> Self {
        self.with(Self::INDEX_NRE, no_reply_expected)
    }

    pub fn with_no_auto_start(&self, no_auto_start: bool) -> Self {
        self.with(Self::INDEX_NAS, no_auto_start)
    }

    pub fn with_allow_interactive_authorization(
        &self,
        allow_interactive_authorization: bool,
    ) -> Self {
        self.with(Self::INDEX_AIA, allow_interactive_authorization)
    }

    fn get(&self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }

    pub fn no_reply_expected(&self) -> bool {
        self.get(Self::INDEX_NRE)
    }

    pub fn no_auto_start(&self) -> bool {
        self.get(Self::INDEX_NAS)
    }

    pub fn allow_interactive_authorization(&self) -> bool {
        self.get(Self::INDEX_AIA)
    }
}

impl WireFormatType for Flags {
    const ALIGNMENT: usize = std::mem::size_of::<u8>();
}

impl WireFormatRead for Flags {
    fn read_from<T: byteorder::ByteOrder, R: std::io::Read>(
        reader: &mut crate::wire_format::MessageReader<R>,
    ) -> std::io::Result<Self> {
        reader.read::<T, u8>().map(Self::from)
    }
}

impl WireFormatWrite for Flags {
    fn write_to<T: byteorder::ByteOrder, W: std::io::Write>(
        &self,
        writer: &mut crate::wire_format::MessageWriter<W>,
    ) -> std::io::Result<()> {
        writer.write::<T, u8>(&self.0)
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        value.0
    }
}
