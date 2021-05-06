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
    pub stack: Stack<Value>,
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
            stack: Stack::empty(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        while let Some(op_code) = self.next_byte()? {
            let instruction = self.find_instruction(op_code)?;
            let arguments_ip = self.current_ip()?.clone();
            self.current_ip()?.jump_forward(instruction.byte_arity);
            (instruction.instruction_fn)(self, arguments_ip)?;
        }
        Ok(())
    }

    fn next_byte(&mut self) -> Result<Option<u8>, MachineError> {
        let code = self.code;
        let ip = self.current_ip()?;
        code.read(ip)
    }

    fn current_ip(&mut self) -> Result<&mut InstructionPointer, MachineError> {
        if let Some(ip) = self.stack.current_ip() {
            Ok(ip)
        } else {
            Err(MachineError(
                "Current instruction pointer was null".to_string(),
            ))
        }
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
