use std::{io::Read, os::unix::net::UnixStream};

use byteorder::{ByteOrder, ReadBytesExt as _};

use crate::wire_format::{StringLengthType, WireFormatRead, WireFormatType};

#[derive(Debug)]
pub struct MessageReader<'a, R: Read> {
    pub(super) stream: &'a mut R,
    pub(super) offset: usize,
}

impl<'a> MessageReader<'a, UnixStream> {
    pub fn new(stream: &'a mut UnixStream) -> Self {
        Self { stream, offset: 0 }
    }
}

impl<'a, R: Read> MessageReader<'a, R> {
    pub fn align_to(&mut self, alignment: usize) -> std::io::Result<()> {
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

    pub fn is_aligned_to(&self, alignment: usize) -> bool {
        self.offset.is_multiple_of(alignment)
    }

    pub fn read_byte(&mut self) -> std::io::Result<u8> {
        self.read_u8()
    }

    pub fn read<T: ByteOrder, E: WireFormatRead>(&mut self) -> std::io::Result<E> {
        E::read_from::<T, _>(self)
    }

    pub fn read_body(mut self, length: usize) -> std::io::Result<Vec<u8>> {
        const BODY_ALIGNMENT: usize = 8;
        self.align_to(BODY_ALIGNMENT)?;
        let mut body = vec![0u8; length];
        self.stream.read_exact(&mut body)?;
        self.offset += length;
        Ok(body)
    }

    pub(super) fn read_bool<T: ByteOrder>(&mut self) -> std::io::Result<bool> {
        match self.read_u32::<T>()? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid boolean value: {}", x),
            )),
        }
    }

    pub(super) fn read_u8(&mut self) -> std::io::Result<u8> {
        let res = self.stream.read_u8()?;
        self.offset += std::mem::size_of::<u8>();
        Ok(res)
    }

    pub(super) fn read_u16<T: ByteOrder>(&mut self) -> std::io::Result<u16> {
        self.align_to(<u16 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_u16::<T>()?;
        self.offset += std::mem::size_of::<u16>();
        Ok(res)
    }

    pub(super) fn read_i16<T: ByteOrder>(&mut self) -> std::io::Result<i16> {
        self.align_to(<i16 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_i16::<T>()?;
        self.offset += std::mem::size_of::<i16>();
        Ok(res)
    }

    pub(super) fn read_u32<T: ByteOrder>(&mut self) -> std::io::Result<u32> {
        self.align_to(<u32 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_u32::<T>()?;
        self.offset += std::mem::size_of::<u32>();
        Ok(res)
    }

    pub(super) fn read_i32<T: ByteOrder>(&mut self) -> std::io::Result<i32> {
        self.align_to(<i32 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_i32::<T>()?;
        self.offset += std::mem::size_of::<i32>();
        Ok(res)
    }

    pub(super) fn read_u64<T: ByteOrder>(&mut self) -> std::io::Result<u64> {
        self.align_to(<u64 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_u64::<T>()?;
        self.offset += std::mem::size_of::<u64>();
        Ok(res)
    }

    pub(super) fn read_i64<T: ByteOrder>(&mut self) -> std::io::Result<i64> {
        self.align_to(<i64 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_i64::<T>()?;
        self.offset += std::mem::size_of::<i64>();
        Ok(res)
    }

    pub(super) fn read_f64<T: ByteOrder>(&mut self) -> std::io::Result<f64> {
        self.align_to(<f64 as WireFormatType>::ALIGNMENT)?;
        let res = self.stream.read_f64::<T>()?;
        self.offset += std::mem::size_of::<f64>();
        Ok(res)
    }

    pub(super) fn read_string<T: ByteOrder, L: StringLengthType>(
        &mut self,
    ) -> std::io::Result<String> {
        let length = self.read::<T, L>()?.to_usize();
        let bytes = self.read_bytes(length)?;
        let string = String::from_utf8(bytes).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "not a valid utf8 string")
        })?;
        let null_byte = self.read_byte()?;
        if null_byte != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "missing null byte at the end of string",
            ));
        }
        Ok(string)
    }

    pub(super) fn read_array<T: ByteOrder, E: WireFormatRead>(
        &mut self,
    ) -> std::io::Result<Vec<E>> {
        let byte_length = self.read_u32::<T>()?;
        self.align_to(E::ALIGNMENT)?;

        if byte_length as usize > 2usize.pow(26) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "array (len: {}) exceeding max allowed length of 2^26 bytes (64 MiB)",
                    byte_length,
                ),
            ));
        }

        let start_offset = self.offset;
        let end_offset = start_offset + byte_length as usize;

        let mut array = Vec::new();

        while self.offset < end_offset {
            let element = E::read_from::<T, _>(self)?;
            array.push(element);
        }

        Ok(array)
    }

    fn read_bytes(&mut self, count: usize) -> std::io::Result<Vec<u8>> {
        let mut bytes = vec![0u8; count];
        self.stream.read_exact(&mut bytes)?;
        self.offset += bytes.len();
        Ok(bytes)
    }
}
