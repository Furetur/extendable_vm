use crate::chunk::{Chunk, Instruction};
use crate::stack::Stack;
use crate::jexvalues::{JexValue, are_values_equal};

type Value = i8;

pub struct VM {
    stack: Stack
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Stack::new_empty()
        }
    }
    pub fn run(&mut self, chunk: &Chunk) -> JexValue {
        for instruction in &chunk.code {
            println!("Running {:?}", instruction);
            self.run_instruction(chunk, instruction)
        }
        self.stack.peek().unwrap()
    }

    fn run_instruction(&mut self, chunk: &Chunk, instruction: &Instruction) {
        match instruction {
            Instruction::CONSTANT(index) => {
                let index = usize::from(*index);
                let value = chunk.constants.get(index).unwrap();
                println!("Putting {} into stack", value);
                self.stack.push(JexValue::INT(*value))
            }
            Instruction::NULL => self.stack.push(JexValue::NULL),
            Instruction::TRUE => self.stack.push(JexValue::BOOLEAN(true)),
            Instruction::FALSE => self.stack.push(JexValue::BOOLEAN(false)),
            Instruction::NOT => {
                let value = self.stack.pop().unwrap();
                match value {
                    JexValue::BOOLEAN(value) => self.stack.push(JexValue::BOOLEAN(!value)),
                    value => panic!("NOT not supported for {:?}", value)
                }
            }
            Instruction::EQUAL => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(JexValue::BOOLEAN(are_values_equal(&left, &right)))
            }
            Instruction::GREATER => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                match (left, right) {
                    (JexValue::INT(a), JexValue::INT(b)) => {
                        self.stack.push(JexValue::BOOLEAN(a > b))
                    }
                    (x, y) => panic!("GREATER not supported for {:?} and {:?}", x, y)
                }
            }
            Instruction::LESS => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                match (left, right) {
                    (JexValue::INT(a), JexValue::INT(b)) => {
                        self.stack.push(JexValue::BOOLEAN(a < b))
                    }
                    (x, y) => panic!("LESS not supported for {:?} and {:?}", x, y)
                }
            }
            Instruction::NEGATE => {
                let value = self.stack.pop().unwrap();
                match value {
                    JexValue::INT(value) => self.stack.push(JexValue::INT(-value)),
                    value => panic!("NEGATE not supported for {:?}", value)
                }
            }
            Instruction::ADD => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                match (left, right) {
                    (JexValue::INT(a), JexValue::INT(b)) => {
                        self.stack.push(JexValue::INT(a + b))
                    }
                    (x, y) => panic!("ADD not supported for {:?} and {:?}", x, y)
                }
            },
            Instruction::SUBTRACT => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                match (left, right) {
                    (JexValue::INT(a), JexValue::INT(b)) => {
                        self.stack.push(JexValue::INT(a - b))
                    }
                    (x, y) => panic!("SUBTRACT not supported for {:?} and {:?}", x, y)
                }
            },
            Instruction::MULTIPLY => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                match (left, right) {
                    (JexValue::INT(a), JexValue::INT(b)) => {
                        self.stack.push(JexValue::INT(a * b))
                    }
                    (x, y) => panic!("MULTIPLY not supported for {:?} and {:?}", x, y)
                }
            },
            Instruction::DIVIDE => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                match (left, right) {
                    (JexValue::INT(a), JexValue::INT(b)) => {
                        self.stack.push(JexValue::INT(a / b))
                    }
                    (x, y) => panic!("DIVIDE not supported for {:?} and {:?}", x, y)
                }
            }
            _ => (),
        }
    }
}
