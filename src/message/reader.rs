use std::{io::Read, os::unix::net::UnixStream};

use byteorder::{ByteOrder, ReadBytesExt as _};

use crate::wire_format::WireFormatType;

#[derive(Debug)]
pub struct MessageReader<'a, R: Read> {
    stream: &'a mut R,
    offset: usize,
}

impl<'a> MessageReader<'a, UnixStream> {
    pub fn new(stream: &'a mut UnixStream) -> Self {
        Self { stream, offset: 0 }
    }
}

impl<'a, R: Read> MessageReader<'a, R> {
    fn align(&mut self, alignment: usize) -> std::io::Result<()> {
        let mut padding_buffer = [0u8; 8];
        let remainder = self.offset % alignment;
        if remainder != 0 {
            let padding_bytes = alignment - remainder;
            self.stream
                .read_exact(&mut padding_buffer[..padding_bytes])?;
            self.offset += padding_bytes;
        }
        Ok(())
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        let res = self.stream.read_u8()?;
        self.offset += 1;
        Ok(res)
    }

    pub fn read_u32<T: ByteOrder>(&mut self) -> std::io::Result<u32> {
        const BYTES: usize = std::mem::size_of::<u32>();
        self.align(BYTES)?;
        let res = self.stream.read_u32::<T>()?;
        self.offset += BYTES;
        Ok(res)
    }

    pub fn read_array<T: ByteOrder, E: WireFormatType>(&mut self) -> std::io::Result<Vec<E>> {
        todo!();
    }

    pub fn read_body(mut self, length: usize) -> std::io::Result<Vec<u8>> {
        self.align(8)?;
        let mut body = vec![0u8; length];
        self.stream.read_exact(&mut body)?;
        self.offset += length;
        Ok(body)
    }
}
