use crate::Chunk;

#[derive(Clone)]
pub struct InstructionPointer {
    pub chunk_id: usize,
    pub instruction_pointer: usize,
}

impl InstructionPointer {
    pub fn new(chunk_id: usize) -> InstructionPointer {
        InstructionPointer {
            instruction_pointer: 0,
            chunk_id,
        }
    }

    pub fn read_and_advance<Value>(&mut self, chunk: &Chunk<Value>) -> Option<u8> {
        if self.instruction_pointer < chunk.code.len() {
            let instruction = chunk.code[self.instruction_pointer];
            self.instruction_pointer += 1;
            Some(instruction)
        } else {
            None
        }
    }

    pub fn jump_forward(&mut self, offset: usize) {
        self.instruction_pointer += offset
    }

    pub fn jump_backward(&mut self, offset: usize) {
        if self.instruction_pointer < offset {
            panic!("Jumped too far backward: ip={}", self.instruction_pointer);
        } else {
            self.instruction_pointer -= offset;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Chunk, InstructionPointer};

    #[test]
    fn should_iterate_over_all_instructions() {
        let mut expected_code: Vec<u8> = vec![];
        for i in 1..100 {
            expected_code.push(i);
        }

        let chunk = Chunk {
            constants: vec![0],
            code: expected_code.clone(),
        };

        let mut pointer = InstructionPointer::new(0);
        let mut actual_code: Vec<u8> = vec![];

        while let Some(byte) = pointer.read_and_advance(&chunk) {
            actual_code.push(byte)
        }
        assert_eq!(expected_code, actual_code)
    }

    #[test]
    fn should_jump_backward() {
        let code: Vec<u8> = vec![0, 1, 2, 3, 4, 5];

        let chunk = Chunk {
            constants: vec![0],
            code,
        };

        let expected_code: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 2, 3, 4, 5];

        let mut pointer = InstructionPointer::new(0);
        let mut actual_code: Vec<u8> = vec![];

        while let Some(byte) = pointer.read_and_advance(&chunk) {
            actual_code.push(byte)
        }
        pointer.jump_backward(4);
        while let Some(byte) = pointer.read_and_advance(&chunk) {
            actual_code.push(byte)
        }

        assert_eq!(expected_code, actual_code)
    }

    #[test]
    fn should_jump_forward() {
        let chunk = Chunk {
            constants: vec![0],
            code: vec![0, 1, 2, 3, 4, 5],
        };

        let mut pointer = InstructionPointer::new(0);
        pointer.jump_forward(5);

        assert_eq!(5, pointer.read_and_advance(&chunk).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_panic_if_jumps_too_far_backward() {
        let mut pointer = InstructionPointer::new(0);
        pointer.jump_backward(10);
    }
}
