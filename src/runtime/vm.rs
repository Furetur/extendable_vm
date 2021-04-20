use crate::bytecode::chunk::{Chunk, ChunkConstant};
use crate::bytecode::instructions::Instruction;
use crate::runtime::vm_reader::VmReader;
use crate::string_interner::StringInterner;
use crate::values::jex_object::RawObject;
use crate::values::jex_values::{are_values_equal, JexValue};
use std::collections::HashMap;

pub struct VM {
    stack: Vec<JexValue>,
    string_interner: StringInterner,
    globals: HashMap<String, JexValue>,
    reader: VmReader,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Vec::new(),
            string_interner: StringInterner::new(),
            globals: HashMap::new(),
            reader: VmReader::new(),
        }
    }
    pub fn run(&mut self, chunk: &Chunk) -> Option<JexValue> {
        self.reader = VmReader::new();
        while let Some(instruction) = self.reader.next(chunk) {
            println!("Running {:?}", instruction);
            self.run_instruction(chunk, instruction);
            println!("\tStack: {:?}", &self.stack);
        }
        self.stack.pop()
    }

    fn run_instruction(&mut self, chunk: &Chunk, instruction: &Instruction) {
        match instruction {
            Instruction::Constant(index) => self.run_constant_instruction(chunk, *index),
            Instruction::DefineGlobal(index) => self.run_define_global_instruction(chunk, *index),
            Instruction::GetGlobal(index) => self.run_get_global(chunk, *index),
            // TODO: actually make a distinction between define and set
            Instruction::SetGlobal(index) => self.run_define_global_instruction(chunk, *index),
            Instruction::GetLocal(slot) => self.run_get_local(*slot),
            Instruction::SetLocal(slot) => self.run_set_local(*slot),
            Instruction::Pop => self.run_pop(),
            Instruction::Null => self.run_null_instruction(),
            Instruction::True => self.run_boolean_instruction(true),
            Instruction::False => self.run_boolean_instruction(false),
            Instruction::Not => self.run_not_instruction(),
            Instruction::Equal => self.run_equal_instruction(),
            Instruction::Greater => self.run_greater_instruction(),
            Instruction::Less => self.run_less_instruction(),
            Instruction::Negate => self.run_negate_instruction(),
            Instruction::Add => self.run_add_instruction(),
            Instruction::Subtract => self.run_subtract_instruction(),
            Instruction::Multiply => self.run_multiply_instruction(),
            Instruction::Divide => self.run_divide_instruction(),
            Instruction::Print => self.run_print_instruction(),
            Instruction::JumpForward(offset) => self.run_jump_forward(*offset),
            Instruction::JumpForwardIfFalse(offset) => self.run_jump_forward_if_false(*offset),
            Instruction::JumpBackward(offset) => self.run_jump_backward(*offset),
        }
    }

    fn run_constant_instruction(&mut self, chunk: &Chunk, constant_index: usize) {
        let constant_value = chunk.constants.get(constant_index).unwrap();
        let jex_value: JexValue = constant_value.to_jex_value();
        println!("Putting {:?} into stack", &jex_value);
        self.push_into_stack(jex_value);
    }

    fn read_string_constant(&self, chunk: &Chunk, constant_index: usize) -> String {
        let constant = chunk.constants.get(constant_index).unwrap();
        match constant {
            ChunkConstant::STRING(s) => s.clone(),
            _ => panic!("Constant {:?} was not String", constant),
        }
    }

    fn run_define_global_instruction(&mut self, chunk: &Chunk, name_index: usize) {
        let name = self.read_string_constant(chunk, name_index);
        let value = self.get_operand();
        self.globals.insert(name, value);
    }

    fn run_get_global(&mut self, chunk: &Chunk, constant_index: usize) {
        let name = self.read_string_constant(chunk, constant_index);
        let value = self.globals.get(&name);
        if let Some(value) = value {
            self.push_into_stack(value.clone());
        } else {
            panic!("Global not found");
        }
    }

    fn run_set_local(&mut self, slot: usize) {
        if self.stack.len() <= slot {
            panic!(
                "Cannot SET local variable with slot {} because stack length is {}",
                slot,
                self.stack.len()
            );
        } else {
            self.stack[slot] = self.get_operand();
        }
    }

    fn run_get_local(&mut self, slot: usize) {
        if self.stack.len() <= slot {
            panic!(
                "Cannot GET local variable with slot {} because stack length is {}",
                slot,
                self.stack.len()
            );
        } else {
            self.push_into_stack(self.stack[slot].clone());
        }
    }

    fn run_pop(&mut self) {
        self.get_operand();
    }

    fn run_null_instruction(&mut self) {
        self.push_into_stack(JexValue::NULL);
    }

    fn run_boolean_instruction(&mut self, value: bool) {
        self.push_into_stack(JexValue::BOOLEAN(value));
    }

    fn run_not_instruction(&mut self) {
        let value = self.get_operand();
        let new_value = match value {
            JexValue::BOOLEAN(value) => JexValue::BOOLEAN(!value),
            value => panic!("NOT not supported for {:?}", value),
        };
        self.push_into_stack(new_value);
    }

    fn run_equal_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        self.push_into_stack(JexValue::BOOLEAN(are_values_equal(&left, &right)))
    }

    fn run_greater_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => self.push_into_stack(JexValue::BOOLEAN(a > b)),
            (x, y) => panic!("GREATER not supported for {:?} and {:?}", x, y),
        }
    }

    fn run_less_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => self.push_into_stack(JexValue::BOOLEAN(a < b)),
            (x, y) => panic!("LESS not supported for {:?} and {:?}", x, y),
        }
    }

    fn run_negate_instruction(&mut self) {
        let value = self.get_operand();
        match value {
            JexValue::INT(value) => self.push_into_stack(JexValue::INT(-value)),
            value => panic!("NEGATE not supported for {:?}", value),
        }
    }

    fn run_add_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => self.push_into_stack(JexValue::INT(a + b)),
            (JexValue::OBJECT(o1), JexValue::OBJECT(o2)) => match (&*o1, &*o2) {
                (RawObject::STRING(s1), RawObject::STRING(s2)) => {
                    let mut result_str = s1.clone();
                    result_str.push_str(&s2);
                    let value = self.string_interner.get_string_value(result_str);
                    self.push_into_stack(value);
                }
                _ => panic!("ADD not supported"),
            },
            (x, y) => panic!("ADD not supported for {:?} and {:?}", x, y),
        }
    }

    fn run_subtract_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => self.push_into_stack(JexValue::INT(a - b)),
            (x, y) => panic!("SUBTRACT not supported for {:?} and {:?}", x, y),
        }
    }

    fn run_multiply_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => self.push_into_stack(JexValue::INT(a * b)),
            (x, y) => panic!("MULTIPLY not supported for {:?} and {:?}", x, y),
        }
    }

    fn run_divide_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => self.push_into_stack(JexValue::INT(a / b)),
            (x, y) => panic!("DIVIDE not supported for {:?} and {:?}", x, y),
        }
    }

    fn run_print_instruction(&mut self) {
        let value = self.get_operand();
        println!(">>>PRINTING: {}", value.to_output_string())
    }

    fn run_jump_forward_if_false(&mut self, offset: usize) {
        let bool = self.get_operand().as_bool();
        if !bool {
            self.run_jump_forward(offset);
        }
    }

    fn run_jump_forward(&mut self, offset: usize) {
        self.reader.jump_forward(offset);
    }

    fn run_jump_backward(&mut self, offset: usize) {
        self.reader.jump_backward(offset);
    }

    fn push_into_stack(&mut self, value: JexValue) {
        self.stack.push(value)
    }

    fn get_two_operands(&mut self) -> (JexValue, JexValue) {
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();
        (left, right)
    }
    fn get_operand(&mut self) -> JexValue {
        self.stack.pop().unwrap()
    }
}
