use std::fs;
use std::io::{Error, ErrorKind};

pub struct BytecodeReader {
    bytes: Vec<u8>,
    next_byte: usize,
}

impl BytecodeReader {
    pub fn from_file(path: &String) -> Result<BytecodeReader, Error> {
        let bytes = fs::read(path)?;
        Ok(BytecodeReader::new(bytes))
    }
    pub fn new(bytes: Vec<u8>) -> BytecodeReader {
        BytecodeReader {
            bytes,
            next_byte: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.next_byte
    }
    ///
    /// Returns Some(Vec) if [n_bytes] were read successfully
    /// Returns Err([n]) if there was an error reading [n]th byte.
    pub fn read_bytes(&mut self, n_bytes: usize) -> Result<Vec<u8>, usize> {
        let mut result: Vec<u8> = vec![];
        for _ in 0..n_bytes {
            result.push(self.read_byte()?)
        }
        Ok(result)
    }
    pub fn read_usize(&mut self) -> Result<usize, usize> {
        Ok(usize::from(self.read_byte()?))
    }
    pub fn read_byte(&mut self) -> Result<u8, usize> {
        if self.is_finished() {
            Err(self.next_byte)
        } else {
            let byte = self.bytes[self.next_byte];
            self.next_byte += 1;
            Ok(byte)
        }
    }
    pub fn is_finished(&self) -> bool {
        self.next_byte >= self.bytes.len()
    }
}
