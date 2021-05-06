use crate::machine::instruction_pointer::InstructionPointer;
use std::fmt::Debug;
use crate::machine::errors::MachineError;

pub struct Stack<Value: Debug> {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
}

struct CallFrame {
    chunk_id: usize,
    instruction_pointer: InstructionPointer,
    start_slot: usize,
}

impl CallFrame {
    fn new(chunk_id: usize, start_slot: usize) -> CallFrame {
        CallFrame {
            chunk_id,
            instruction_pointer: InstructionPointer::new(chunk_id),
            start_slot,
        }
    }
}

impl<Value: Debug> Stack<Value> {
    pub fn empty() -> Stack<Value> {
        Stack {
            stack: vec![],
            frames: vec![],
        }
    }

    pub fn current_ip(&mut self) -> Option<&mut InstructionPointer> {
        self.frames.last_mut().map(|frame| &mut frame.instruction_pointer)
    }

    pub fn push_call_frame(&mut self, chunk_id: usize, arity: usize) -> Result<(), MachineError> {
        if self.stack.len() > arity {
            let start_slot = self.stack.len() - arity - 1;
            let frame = CallFrame::new(chunk_id, start_slot);
            self.frames.push(frame);
            Ok(())
        } else {
            let message = format!("Cannot PUSH frame with arity {} when stack length is {}", arity, self.stack.len());
            Err(MachineError(message))
        }
    }

    pub fn discard_call_frame(&mut self) -> Result<(), MachineError> {
        let last_frame = self.frames.pop();

        if let Some(last_frame) = last_frame {
            let last_frame_start = last_frame.start_slot;
            while self.has_slot(last_frame_start) {
                self.stack.pop();
            }
            Ok(())
        } else {
            Err(MachineError("Cannot discard call frame because there are no call frames".to_string()))
        }
    }

    pub fn get_local(&self, local_slot: usize) -> Result<&Value, MachineError> {
        let absolute_slot = self.last_frame_start_slot()? + local_slot;
        if self.has_slot(absolute_slot) {
            Ok(&self.stack[absolute_slot])
        } else {
            let message = format!(
                "Tried to GET local value with absolute_slot={} when stack length was {}",
                absolute_slot,
                self.stack.len()
            );
            Err(MachineError(message))
        }
    }

    pub fn set_local(&mut self, local_slot: usize, value: Value) -> Result<(), MachineError> {
        let absolute_slot = self.last_frame_start_slot()? + local_slot;
        if self.has_slot(absolute_slot) {
            self.stack[absolute_slot] = value;
            Ok(())
        } else {
            let message = format!(
                "Tried to SET local value with absolute_slot={} when stack length was {}",
                absolute_slot,
                self.stack.len()
            );
            Err(MachineError(message))
        }
    }

    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }

    pub fn peek_from_top(&self, offset: usize) -> Result<&Value, MachineError> {
        if self.stack.len() > offset {
            Ok(&self.stack[self.stack.len() - 1 - offset])
        } else {
            let message = format!(
                "Cannot peek from top with offset {} when stack length is {}",
                offset,
                self.stack.len()
            );
            Err(MachineError(message))
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop_two_operands(&mut self) -> Result<(Value, Value), MachineError> {
        let right = self.pop()?;
        let left = self.pop()?;
        Ok((left, right))
    }

    pub fn pop(&mut self) -> Result<Value, MachineError> {
        let last_frame_start = self.last_frame_start_slot()?;
        let value = self.stack.pop();
        if !self.has_slot(last_frame_start) {
            let message = format!(
                "Attempted to manually pop the last value of the call frame with start={}",
                last_frame_start
            );
            Err(MachineError(message))
        } else if let Some(existing_value) = value {
            Ok(existing_value)
        } else {
            Err(MachineError(
                "Attempted to pop the value from an empty stack".to_string(),
            ))
        }
    }

    fn has_slot(&self, slot: usize) -> bool {
        slot < self.stack.len()
    }

    fn last_frame_start_slot(&self) -> Result<usize, MachineError> {
        if let Some(frame) = self.frames.last() {
            Ok(frame.start_slot)
        } else {
            Err(MachineError("There were no call frames!".to_string()))
        }
    }

    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self.stack)
    }
}
