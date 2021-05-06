use crate::machine::errors::MachineError;
use std::fs;
use std::io::{Error, ErrorKind};

pub struct BytecodeReader {
    bytes: Vec<u8>,
    next_byte: usize,
}

impl BytecodeReader {
    pub fn from_file(path: &String) -> Result<BytecodeReader, MachineError> {
        let bytes = fs::read(path).map_err(|e| MachineError("File not found".to_string()))?;
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
    pub fn read_bytes(&mut self, n_bytes: usize, for_what: &str) -> Result<Vec<u8>, MachineError> {
        let mut result: Vec<u8> = vec![];
        for _ in 0..n_bytes {
            result.push(self.read_byte(for_what)?)
        }
        Ok(result)
    }
    pub fn read_byte(&mut self, for_what: &str) -> Result<u8, MachineError> {
        if self.is_finished() {
            let message = format!("Bytecode ended unexpectedly while reading {}", for_what);
            Err(MachineError(message))
        } else {
            let byte = self.bytes[self.next_byte];
            self.next_byte += 1;
            Ok(byte)
        }
    }
    pub fn read_u16(&mut self, for_what: &str) -> Result<u16, MachineError> {
        let bytes = self.read_bytes(2, for_what)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }
    pub fn read_u32(&mut self, for_what: &str) -> Result<u32, MachineError> {
        let bytes = self.read_bytes(4, for_what)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    pub fn read_i32(&mut self, for_what: &str) -> Result<i32, MachineError> {
        let bytes = self.read_bytes(4, for_what)?;
        Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    pub fn is_finished(&self) -> bool {
        self.next_byte >= self.bytes.len()
    }
}
