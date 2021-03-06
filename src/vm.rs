use crate::chunk::{Chunk, Instruction};
use crate::jexvalues::{JexValue, are_values_equal};
use crate::jexobject::RawObject;
use crate::string_interner::StringInterner;

pub struct VM {
    stack: Vec<JexValue>,
    string_interner: StringInterner,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Vec::new(),
            string_interner: StringInterner::new()
        }
    }
    pub fn run(&mut self, chunk: &Chunk) -> Option<JexValue> {
        for instruction in &chunk.code {
            println!("Running {:?}", instruction);
            self.run_instruction(chunk, instruction)
        }
        self.stack.pop()
    }

    fn run_instruction(&mut self, chunk: &Chunk, instruction: &Instruction) {
        match instruction {
            Instruction::CONSTANT(index) => self.run_constant_instruction(chunk, *index),
            Instruction::NULL => self.run_null_instruction(),
            Instruction::TRUE => self.run_boolean_instruction(true),
            Instruction::FALSE => self.run_boolean_instruction(false),
            Instruction::NOT => self.run_not_instruction(),
            Instruction::EQUAL => self.run_equal_instruction(),
            Instruction::GREATER => self.run_greater_instruction(),
            Instruction::LESS => self.run_less_instruction(),
            Instruction::NEGATE => self.run_negate_instruction(),
            Instruction::ADD => self.run_add_instruction(),
            Instruction::SUBTRACT => self.run_subtract_instruction(),
            Instruction::MULTIPLY => self.run_multiply_instruction(),
            Instruction::DIVIDE => self.run_divide_instruction(),
            Instruction::PRINT => self.run_print_instruction(),
        }
    }

    fn run_constant_instruction(&mut self, chunk: &Chunk, constant_index: usize) {
        let constant_value = chunk.constants.get(constant_index).unwrap();
        let jex_value: JexValue = constant_value.to_jex_value();
        println!("Putting {:?} into stack", &jex_value);
        self.push_into_stack(jex_value);
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
            value => panic!("NOT not supported for {:?}", value)
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
            (JexValue::INT(a), JexValue::INT(b)) => {
                self.push_into_stack(JexValue::BOOLEAN(a > b))
            }
            (x, y) => panic!("GREATER not supported for {:?} and {:?}", x, y)
        }
    }

    fn run_less_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => {
                self.push_into_stack(JexValue::BOOLEAN(a < b))
            }
            (x, y) => panic!("LESS not supported for {:?} and {:?}", x, y)
        }
    }

    fn run_negate_instruction(&mut self) {
        let value = self.get_operand();
        match value {
            JexValue::INT(value) => self.push_into_stack(JexValue::INT(-value)),
            value => panic!("NEGATE not supported for {:?}", value)
        }
    }

    fn run_add_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => {
                self.push_into_stack(JexValue::INT(a + b))
            }
            (JexValue::OBJECT(o1), JexValue::OBJECT(o2)) => {
                match (&*o1, &*o2) {
                    (RawObject::STRING(s1), RawObject::STRING(s2)) => {
                        let mut result_str = s1.clone();
                        result_str.push_str(&s2);
                        let value = self.string_interner.get_string_value(result_str);
                        self.push_into_stack(value);
                    }
                    _ => panic!("ADD not supported")
                }
            }
            (x, y) => panic!("ADD not supported for {:?} and {:?}", x, y)
        }
    }

    fn run_subtract_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => {
                self.push_into_stack(JexValue::INT(a - b))
            }
            (x, y) => panic!("SUBTRACT not supported for {:?} and {:?}", x, y)
        }
    }

    fn run_multiply_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => {
                self.push_into_stack(JexValue::INT(a * b))
            }
            (x, y) => panic!("MULTIPLY not supported for {:?} and {:?}", x, y)
        }
    }

    fn run_divide_instruction(&mut self) {
        let (left, right) = self.get_two_operands();
        match (left, right) {
            (JexValue::INT(a), JexValue::INT(b)) => {
                self.push_into_stack(JexValue::INT(a / b))
            }
            (x, y) => panic!("DIVIDE not supported for {:?} and {:?}", x, y)
        }
    }

    fn run_print_instruction(&mut self) {
        let value = self.get_operand();
        println!("PRINTING: {}", value.to_output_string())
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
