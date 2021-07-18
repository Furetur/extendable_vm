pub trait ByteReadable<Ptr> {
    fn read(&self, ptr: &mut Ptr) -> Option<u8>;
    fn has_next(&self, ptr: &Ptr) -> bool;

    fn read_n(&self, ptr: &mut Ptr, n: usize) -> Option<Vec<u8>> {
        let mut result: Vec<u8> = Vec::new();
        for _ in 0..n {
            let byte = self.read(ptr)?;
            result.push(byte)
        }
        Some(result)
    }

    fn read_u16(&self, ptr: &mut Ptr) -> Option<u16> {
        let bytes = self.read_n(ptr, 2)?;
        Some(u16::from_le_bytes([bytes[0], bytes[1]]))
    }
    fn read_u32(&self, ptr: &mut Ptr) -> Option<u32> {
        let bytes = self.read_n(ptr, 4)?;
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    fn read_i32(&self, ptr: &mut Ptr) -> Option<i32> {
        let bytes = self.read_n(ptr, 4)?;
        Some(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}
