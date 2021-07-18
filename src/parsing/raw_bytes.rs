use crate::byte_readable::ByteReadable;
use std::fs;
use std::io::Error;

pub struct RawBytesPointer {
    pub next_byte: usize,
}

impl RawBytesPointer {
    pub fn new() -> RawBytesPointer {
        RawBytesPointer { next_byte: 0 }
    }
}

impl Default for RawBytesPointer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RawBytes {
    data: Vec<u8>,
}

impl RawBytes {
    pub fn from_file(path: &str) -> Result<RawBytes, Error> {
        let data = fs::read(path)?;
        Ok(RawBytes { data })
    }
    pub fn from_bytes(bytes: Vec<u8>) -> RawBytes {
        RawBytes { data: bytes }
    }
}

impl ByteReadable<RawBytesPointer> for RawBytes {
    fn read(&self, ptr: &mut RawBytesPointer) -> Option<u8> {
        let result = self.data.get(ptr.next_byte).cloned();
        ptr.next_byte += 1;
        result
    }

    fn has_next(&self, ptr: &RawBytesPointer) -> bool {
        ptr.next_byte < self.data.len()
    }
}
