use crate::values::jex_values::JexValue;
use std::borrow::Borrow;
use crate::runtime::stack_errors::SlotDoesNotExistError;

pub struct Stack {
    stack: Vec<JexValue>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: vec![]
        }
    }
    pub fn peek(&self) -> Option<&JexValue> {
        self.stack.last()
    }
    pub fn copy_from_slot_to_top(&mut self, slot: usize) -> Result<(), SlotDoesNotExistError> {
        let value = self.get_value(slot);
        if let Some(value) = value {
            self.push(value.clone());
            Ok(())
        } else {
            Err(SlotDoesNotExistError { stack_len: self.stack.len(), slot })
        }
    }
    pub fn pop_and_put_into_slot(&mut self, slot: usize) -> Result<(), SlotDoesNotExistError> {
        let value = self.pop();
        if let Some(value) = value {
            self.set_value(slot, value)?;
            Ok(())
        } else {
            Err(SlotDoesNotExistError { stack_len: self.stack.len(), slot })
        }
    }
    pub fn set_value(&mut self, slot: usize, new_value: JexValue) -> Result<(), SlotDoesNotExistError> {
        if self.stack.len() <= slot {
            Err(SlotDoesNotExistError { stack_len: self.stack.len(), slot })
        } else {
            self.stack[slot] = new_value;
            Ok(())
        }
    }
    pub fn get_value(&self, index: usize) -> Option<&JexValue> {
        if self.stack.len() <= index {
            None
        } else {
            Some(&self.stack[index])
        }
    }
    pub fn push(&mut self, value: JexValue) {
        self.stack.push(value);
    }
    pub fn pop_two_operands(&mut self) -> Option<(JexValue, JexValue)> {
        let right = self.pop();
        let left = self.pop();
        match (left, right) {
            (Some(left), Some(right)) => Some((left, right)),
            _ => None,
        }
    }
    pub fn pop(&mut self) -> Option<JexValue> {
        self.stack.pop()
    }
    pub fn to_string(&self) -> String {
        format!("{:?}", self.stack)
    }
}
