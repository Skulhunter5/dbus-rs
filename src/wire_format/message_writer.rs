use std::{io::Write, os::unix::net::UnixStream};

use byteorder::{ByteOrder, WriteBytesExt as _};

use crate::wire_format::{StringLengthType, WireFormatType};

#[derive(Debug)]
pub struct MessageWriter<'a, W: Write> {
    pub(super) stream: &'a mut W,
    pub(super) offset: usize,
}

impl<'a> MessageWriter<'a, UnixStream> {
    pub fn new(stream: &'a mut UnixStream) -> Self {
        Self { stream, offset: 0 }
    }
}

impl<'a, W: Write> MessageWriter<'a, W> {
    pub fn align_to(&mut self, alignment: usize) -> std::io::Result<()> {
        static PADDING_BUFFER: [u8; 8] = [0u8; 8];

        let remainder = self.offset % alignment;
        if remainder != 0 {
            let padding_bytes = alignment - remainder;
            self.stream.write_all(&PADDING_BUFFER[..padding_bytes])?;
            self.offset += padding_bytes;
        }
        Ok(())
    }

    pub fn is_aligned_to(&self, alignment: usize) -> bool {
        self.offset.is_multiple_of(alignment)
    }

    pub fn write_byte(&mut self, value: u8) -> std::io::Result<()> {
        self.write_u8(value)
    }

    pub fn write<T: ByteOrder, E: WireFormatType>(&mut self, value: E) -> std::io::Result<()> {
        value.write_to::<T, _>(self)
    }

    pub fn write_body(mut self, body: &[u8]) -> std::io::Result<()> {
        self.align_to(8)?;
        self.stream.write_all(body)?;
        self.offset += body.len();
        Ok(())
    }

    pub(super) fn write_bool<T: ByteOrder>(&mut self, value: bool) -> std::io::Result<()> {
        self.write_u32::<T>(value as u32)
    }

    pub(super) fn write_u8(&mut self, value: u8) -> std::io::Result<()> {
        self.stream.write_u8(value)?;
        self.offset += 1;
        Ok(())
    }

    pub(super) fn write_u16<T: ByteOrder>(&mut self, value: u16) -> std::io::Result<()> {
        const BYTES: usize = std::mem::size_of::<u16>();
        self.align_to(BYTES)?;
        self.stream.write_u16::<T>(value)?;
        self.offset += BYTES;
        Ok(())
    }

    pub(super) fn write_i16<T: ByteOrder>(&mut self, value: i16) -> std::io::Result<()> {
        const BYTES: usize = std::mem::size_of::<i16>();
        self.align_to(BYTES)?;
        self.stream.write_i16::<T>(value)?;
        self.offset += BYTES;
        Ok(())
    }

    pub(super) fn write_u32<T: ByteOrder>(&mut self, value: u32) -> std::io::Result<()> {
        const BYTES: usize = std::mem::size_of::<u32>();
        self.align_to(BYTES)?;
        self.stream.write_u32::<T>(value)?;
        self.offset += BYTES;
        Ok(())
    }

    pub(super) fn write_i32<T: ByteOrder>(&mut self, value: i32) -> std::io::Result<()> {
        const BYTES: usize = std::mem::size_of::<i32>();
        self.align_to(BYTES)?;
        self.stream.write_i32::<T>(value)?;
        self.offset += BYTES;
        Ok(())
    }

    pub(super) fn write_u64<T: ByteOrder>(&mut self, value: u64) -> std::io::Result<()> {
        const BYTES: usize = std::mem::size_of::<u64>();
        self.align_to(BYTES)?;
        self.stream.write_u64::<T>(value)?;
        self.offset += BYTES;
        Ok(())
    }

    pub(super) fn write_i64<T: ByteOrder>(&mut self, value: i64) -> std::io::Result<()> {
        const BYTES: usize = std::mem::size_of::<i64>();
        self.align_to(BYTES)?;
        self.stream.write_i64::<T>(value)?;
        self.offset += BYTES;
        Ok(())
    }

    pub(super) fn write_string<T: ByteOrder, L: StringLengthType>(
        &mut self,
        string: impl AsRef<str>,
    ) -> std::io::Result<()> {
        let string = string.as_ref();

        self.write_u32::<T>(string.len() as u32)?;
        self.write_bytes(string.as_bytes())?;
        self.write_byte(b'\0')?;

        Ok(())
    }

    pub fn write_array<T: ByteOrder, E: WireFormatType>(
        &mut self,
        array: impl AsRef<[E]>,
    ) -> std::io::Result<()> {
        let array = array.as_ref();

        // TODO: replace the mess of having to write the "header" (i.e. length + element padding) to
        // array_buffer. it should probably be possible to just set array_writer.offset to something
        // different so that it still ensures the correct alignment when writing to the
        // array_buffer. It's probably something like `self.offset + size_of(u32) +
        // element_alignment`
        let array_bytes = {
            let mut array_buffer = Vec::new();
            let mut array_writer = MessageWriter {
                stream: &mut array_buffer,
                offset: self.offset,
            };
            let initial_offset = array_writer.offset;

            array_writer.write_u32::<T>(0)?;
            array_writer.align_to(E::ALIGNMENT)?;

            let header_length = array_writer.offset - initial_offset;

            for element in array {
                element.write_to::<T, _>(&mut array_writer)?;
            }

            let previous_length = array_buffer.len();
            let header_bytes = array_buffer.splice(..header_length, std::iter::empty());
            assert_eq!(header_bytes.count(), header_length);
            assert_eq!(array_buffer.len(), previous_length - header_length);

            array_buffer
        };

        if array_bytes.len() > 2usize.pow(26) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "array (len: {}) exceeding max allowed length of 2^26 bytes (64 MiB)",
                    array_bytes.len()
                ),
            ));
        }

        self.write_u32::<T>(array_bytes.len() as u32)?;
        self.align_to(E::ALIGNMENT)?;
        self.write_bytes(array_bytes)?;

        todo!();
    }

    fn write_bytes(&mut self, bytes: impl AsRef<[u8]>) -> std::io::Result<()> {
        let bytes = bytes.as_ref();
        self.stream.write_all(bytes)?;
        self.offset += bytes.len();
        Ok(())
    }
}
