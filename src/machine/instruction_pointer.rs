use crate::machine::code::Chunk;

#[derive(Clone)]
pub struct InstructionPointer {
    pub chunk_id: usize,
    instruction_pointer: usize,
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
        self.instruction_pointer -= offset;
        if self.instruction_pointer < 0 {
            panic!("Jumped too far backward: ip={}", self.instruction_pointer);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::machine::code::Chunk;
//     use crate::machine::instruction_pointer::InstructionPointer;
//     use crate::machine::instruction_table::Instruction;
//
//     #[test]
//     fn should_iterate_over_all_instructions() {
//         let chunk = Chunk {
//             constants: vec![ChunkConstant::INT(0), ChunkConstant::from_str("str")],
//             code: vec![
//                 Instruction::Constant(0),
//                 Instruction::Constant(1),
//                 Instruction::Add,
//             ],
//         };
//         let expected_instructions = vec![
//             &Instruction::Constant(0),
//             &Instruction::Constant(1),
//             &Instruction::Add,
//         ];
//         let mut reader = InstructionPointer::new();
//         let mut actual_instructions: Vec<&Instruction> = vec![];
//         while let Some(instruction) = reader.read_and_advance(&chunk) {
//             actual_instructions.push(instruction)
//         }
//         assert_eq!(expected_instructions, actual_instructions)
//     }
//
//     #[test]
//     fn should_jump_backward() {
//         let chunk = Chunk {
//             constants: vec![ChunkConstant::INT(0), ChunkConstant::from_str("str")],
//             code: vec![
//                 Instruction::Constant(0),
//                 Instruction::Constant(1),
//                 Instruction::Add,
//             ],
//         };
//         let expected_instructions = vec![
//             &Instruction::Constant(0),
//             &Instruction::Constant(1),
//             &Instruction::Constant(1),
//             &Instruction::Add,
//         ];
//         let mut reader = InstructionPointer::new();
//         let mut actual_instructions: Vec<&Instruction> = vec![];
//         let mut first_time = true;
//         while let Some(instruction) = reader.read_and_advance(&chunk) {
//             if first_time && instruction == &Instruction::Add {
//                 reader.jump_backward(2);
//                 first_time = false;
//                 continue;
//             }
//             actual_instructions.push(instruction);
//         }
//         assert_eq!(expected_instructions, actual_instructions)
//     }
// }
