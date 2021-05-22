use crate::machine::byte_readable::ByteReadable;
use crate::machine::call_frame::CallFrame;
use crate::machine::code::Code;
use crate::machine::exceptions::runtime_exceptions::{
    EmptyCallStack, EmptyOperandStack, SlotOutOfBounds, UnknownOpCode,
};
use crate::machine::exceptions::types::Exception;
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

    pub fn run(&mut self) -> Result<(), Exception> {
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

    pub fn peek_operand(&mut self) -> Result<&Value, EmptyOperandStack> {
        self.operands.peek().ok_or(EmptyOperandStack)
    }

    pub fn pop_operand(&mut self) -> Result<Value, EmptyOperandStack> {
        self.operands.pop().ok_or(EmptyOperandStack)
    }

    pub fn pop_two_operands(&mut self) -> Result<(Value, Value), Exception> {
        let right = self.pop_operand()?;
        let left = self.pop_operand()?;
        Ok((left, right))
    }

    pub fn get_operand(&self, slot: usize) -> Result<&Value, SlotOutOfBounds> {
        self.operands.get(slot).ok_or(SlotOutOfBounds)
    }

    pub fn get_operand_from_top(&self, slot_from_top: usize) -> Result<&Value, SlotOutOfBounds> {
        self.operands
            .get_from_top(slot_from_top)
            .ok_or(SlotOutOfBounds)
    }

    pub fn set_operand(&mut self, slot: usize, value: Value) -> Result<(), SlotOutOfBounds> {
        self.operands.set(slot, value).map_err(|e| SlotOutOfBounds)
    }

    pub fn operand_stack_len(&self) -> usize {
        self.operands.len()
    }

    pub fn peek_frame(&self) -> Result<&CallFrame, EmptyCallStack> {
        self.frames.peek().ok_or(EmptyCallStack)
    }

    pub fn push_frame(&mut self, chunk_id: usize, start_slot: usize) {
        let frame = CallFrame::new(chunk_id, start_slot);
        self.frames.push(frame);
    }

    pub fn discard_frame(&mut self) -> Result<CallFrame, EmptyCallStack> {
        let last_frame = self.frames.pop().ok_or(EmptyCallStack)?;

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

    pub fn instruction_pointer(&mut self) -> Result<&mut InstructionPointer, EmptyCallStack> {
        self.frames
            .peek_mut()
            .map(|frame| &mut frame.instruction_pointer)
            .ok_or(EmptyCallStack)
    }

    fn find_instruction(
        &self,
        op_code: u8,
    ) -> Result<&'a Instruction<Constant, Value>, UnknownOpCode> {
        if let Some(instruction) = self.instruction_table.get_instruction(op_code) {
            Ok(instruction)
        } else {
            Err(UnknownOpCode(op_code))
        }
    }
}

impl<'a, Constant, Value: Debug> ByteReadable<InstructionPointer> for Machine<'a, Constant, Value> {
    fn read(&self, ptr: &mut InstructionPointer) -> Option<u8> {
        self.code.read(ptr)
    }

    fn has_next(&self, ptr: &InstructionPointer) -> bool {
        self.code.has_next(ptr)
    }
}
