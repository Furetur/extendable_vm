use crate::jexvalues::JexValue;

const STACK_MAX: usize = 256;

pub struct Stack {
    stack: [JexValue; STACK_MAX],
    length: usize,
}

impl Stack {
    pub fn new_empty() -> Stack {
        Stack {
            stack: [JexValue::NULL; STACK_MAX],
            length: 0,
        }
    }
    pub fn push(&mut self, value: JexValue) {
        self.stack[self.length] = value;
        self.length += 1;
    }
    pub fn pop(&mut self) -> Option<JexValue> {
        match self.length {
            0 => None,
            _ => {
                self.length -= 1;
                Some(self.stack[self.length])
            }
        }
    }
    pub fn peek(&self) -> Option<JexValue> {
        match self.length {
            0 => None,
            i => Some(self.stack[i - 1])
        }
    }
}
