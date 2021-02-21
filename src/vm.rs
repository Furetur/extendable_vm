use crate::chunk::{Chunk, Instruction};

type Value = u8;

pub struct VM {
    stack: Vec<Value>
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Vec::new()
        }
    }
    pub fn run(&mut self, chunk: &Chunk) -> u8 {
        for instruction in &chunk.code {
            println!("Running {:?}", instruction);
            self.run_instruction(chunk, instruction)
        }
        *self.stack.last().unwrap()
    }

    fn run_instruction(&mut self, chunk: &Chunk, instruction: &Instruction) {
        match instruction {
            Instruction::CONSTANT(index) => {
                let index = usize::from(*index);
                let value = chunk.constants.get(index).unwrap();
                println!("Putting {} into stack", value);
                self.stack.push(*value)
            }
            Instruction::ADD => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(left + right);
            },
            Instruction::SUBTRACT => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(left - right);
            },
            Instruction::MULTIPLY => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(left * right);
            },
            _ => (),
        }
    }
}
