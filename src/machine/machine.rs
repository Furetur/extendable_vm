use crate::machine::byte_readable::ByteReadable;
use crate::machine::call_frame::CallFrame;
use crate::machine::code::Code;
use crate::machine::errors::MachineError;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::{Instruction, InstructionTable};
use crate::machine::stack::Stack;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Machine<'a, Constant, Value: Debug> {
    pub code: &'a Code<Constant>,
    instruction_table: &'a InstructionTable<Constant, Value>,
    operands: Stack<Value>,
    frames: Stack<CallFrame>,
    pub globals: HashMap<String, Value>,
}

impl<'a, Constant, Value: Debug> Machine<'a, Constant, Value> {
    pub fn new(
        code: &'a Code<Constant>,
        instruction_table: &'a InstructionTable<Constant, Value>,
    ) -> Machine<'a, Constant, Value> {
        Machine {
            code,
            instruction_table,
            operands: Stack::empty(),
            frames: Stack::empty(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        while let Some(op_code) = self.next_byte() {
            let instruction = self.find_instruction(op_code)?;
            let arguments_ip = self.instruction_pointer()?.clone();
            self.instruction_pointer()?
                .jump_forward(instruction.byte_arity);
            (instruction.instruction_fn)(self, arguments_ip)?;
        }
        Ok(())
    }

    pub fn push_operand(&mut self, operand: Value) {
        self.operands.push(operand)
    }

    pub fn peek_operand(&mut self) -> Result<&Value, MachineError> {
        self.operands.peek().ok_or(MachineError(
            "Empty stack. Could not get operand".to_string(),
        ))
    }

    pub fn pop_operand(&mut self) -> Result<Value, MachineError> {
        self.operands.pop().ok_or(MachineError(
            "Empty stack. Could not get operand".to_string(),
        ))
    }

    pub fn pop_two_operands(&mut self) -> Result<(Value, Value), MachineError> {
        let right = self.pop_operand()?;
        let left = self.pop_operand()?;
        Ok((left, right))
    }

    pub fn get_operand(&self, slot: usize) -> Result<&Value, MachineError> {
        self.operands
            .get(slot)
            .ok_or(MachineError("Index out of bounds".to_string()))
    }

    pub fn get_operand_from_top(&self, slot_from_top: usize) -> Result<&Value, MachineError> {
        self.operands
            .get_from_top(slot_from_top)
            .ok_or(MachineError("Index out of bounds".to_string()))
    }

    pub fn set_operand(&mut self, slot: usize, value: Value) -> Result<(), MachineError> {
        self.operands
            .set(slot, value)
            .map_err(|e| MachineError("Index out of bounds".to_string()))
    }

    pub fn operand_stack_len(&self) -> usize {
        self.operands.len()
    }

    pub fn peek_frame(&self) -> Result<&CallFrame, MachineError> {
        self.frames
            .peek()
            .ok_or(MachineError("Empty call frame stack".to_string()))
    }

    pub fn push_frame(&mut self, chunk_id: usize, start_slot: usize) {
        let frame = CallFrame::new(chunk_id, start_slot);
        self.frames.push(frame);
    }

    pub fn discard_frame(&mut self) -> Result<CallFrame, MachineError> {
        let last_frame = self.frames.pop().ok_or(MachineError(
            "Cannot discard frame from empty call frame stack!".to_string(),
        ))?;

        let last_frame_start = last_frame.start_slot;
        while self.operands.len() > last_frame_start {
            self.operands.pop();
        }
        Ok(last_frame)
    }

    fn next_byte(&mut self) -> Option<u8> {
        let code = self.code;
        let ip = self.instruction_pointer().ok()?;
        code.read(ip)
    }

    pub fn instruction_pointer(&mut self) -> Result<&mut InstructionPointer, MachineError> {
        self.frames
            .peek_mut()
            .map(|frame| &mut frame.instruction_pointer)
            .ok_or(MachineError("Instruction pointer was None".to_string()))
    }

    fn find_instruction(
        &self,
        op_code: u8,
    ) -> Result<&'a Instruction<Constant, Value>, MachineError> {
        if let Some(instruction) = self.instruction_table.get_instruction(op_code) {
            Ok(instruction)
        } else {
            let message = format!("Unknown instruction with op_code={}", op_code);
            Err(MachineError(message))
        }
    }
}

impl<'a, Constant, Value: Debug> ByteReadable<InstructionPointer> for Machine<'a, Constant, Value> {
    fn read(&self, ptr: &mut InstructionPointer) -> Option<u8> {
        self.code.read(ptr)
    }
}
