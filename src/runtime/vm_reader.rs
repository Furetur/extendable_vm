use crate::bytecode::chunk::Chunk;
use crate::bytecode::instructions::Instruction;

pub struct VmReader {
    ip: usize,
}

impl VmReader {
    pub fn new() -> VmReader {
        VmReader { ip: 0 }
    }

    pub fn next<'a>(&mut self, chunk: &'a Chunk) -> Option<&'a Instruction> {
        if self.ip < chunk.code.len() {
            let instruction = &chunk.code[self.ip];
            self.ip += 1;
            Some(instruction)
        } else {
            None
        }
    }

    pub fn jump_forward(&mut self, offset: usize) {
        self.ip += offset
    }

    pub fn jump_backward(&mut self, offset: usize) {
        self.ip -= offset;
        if self.ip < 0 {
            panic!("Jumped too far backward: ip={}", self.ip);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::chunk::{Chunk, ChunkConstant};
    use crate::bytecode::instructions::Instruction;
    use crate::runtime::vm_reader::VmReader;

    #[test]
    fn should_iterate_over_all_instructions() {
        let chunk = Chunk {
            constants: vec![ChunkConstant::INT(0), ChunkConstant::from_str("str")],
            code: vec![
                Instruction::Constant(0),
                Instruction::Constant(1),
                Instruction::Add,
            ],
        };
        let expected_instructions = vec![
            &Instruction::Constant(0),
            &Instruction::Constant(1),
            &Instruction::Add,
        ];
        let mut reader = VmReader::new();
        let mut actual_instructions: Vec<&Instruction> = vec![];
        while let Some(instruction) = reader.next(&chunk) {
            actual_instructions.push(instruction)
        }
        assert_eq!(expected_instructions, actual_instructions)
    }

    #[test]
    fn should_jump_backward() {
        let chunk = Chunk {
            constants: vec![ChunkConstant::INT(0), ChunkConstant::from_str("str")],
            code: vec![
                Instruction::Constant(0),
                Instruction::Constant(1),
                Instruction::Add,
            ],
        };
        let expected_instructions = vec![
            &Instruction::Constant(0),
            &Instruction::Constant(1),
            &Instruction::Constant(1),
            &Instruction::Add,
        ];
        let mut reader = VmReader::new();
        let mut actual_instructions: Vec<&Instruction> = vec![];
        let mut first_time = true;
        while let Some(instruction) = reader.next(&chunk) {
            if first_time && instruction == &Instruction::Add {
                reader.jump_backward(2);
                first_time = false;
                continue;
            }
            actual_instructions.push(instruction);
        }
        assert_eq!(expected_instructions, actual_instructions)
    }
}
