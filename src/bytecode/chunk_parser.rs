use crate::bytecode::bytecode_reader::BytecodeReader;
use crate::bytecode::chunk::{Chunk, ChunkConstant};
use crate::bytecode::instructions::Instruction;

pub struct ChunkParser<'a> {
    reader: &'a mut BytecodeReader,
    constants: Vec<ChunkConstant>,
    instructions: Vec<Instruction>,
}

impl<'a> ChunkParser<'a> {
    pub fn new(reader: &'a mut BytecodeReader) -> ChunkParser<'a> {
        ChunkParser {
            reader,
            constants: vec![],
            instructions: vec![],
        }
    }
    pub fn parse(mut self) -> Chunk {
        self.parse_constants();
        self.parse_instructions();
        Chunk {
            constants: self.constants,
            code: self.instructions,
        }
    }
    fn parse_constants(&mut self) {
        let n_constants = self.read_byte("number of constants in constant pool");
        for _ in 0..n_constants {
            let constant = self.read_constant();
            self.constants.push(constant);
        }
    }
    fn read_constant(&mut self) -> ChunkConstant {
        let constant_type = self.read_byte("constant type");
        match constant_type {
            0 => self.read_int_constant(),
            1 => self.read_string_constant(),
            i => panic!(
                "Unsupported constant type {} at position {}",
                i,
                self.reader.position()
            ),
        }
    }
    fn read_int_constant(&mut self) -> ChunkConstant {
        let byte = self.read_byte("int constant content");
        ChunkConstant::INT(i8::from_le_bytes([byte]))
    }
    fn read_string_constant(&mut self) -> ChunkConstant {
        let str_size = self.read_usize("string constant size");
        let bytes = self.read_bytes(str_size, "string constant content");
        ChunkConstant::STRING(String::from_utf8(bytes).expect("Could not decode string from bytes"))
    }
    fn parse_instructions(&mut self) {
        while !self.reader.is_finished() {
            self.parse_instruction();
        }
    }
    fn parse_instruction(&mut self) {
        let op_code = self.read_byte("OPCODE");
        let instruction = match op_code {
            0 => Instruction::Constant(self.read_usize("Instruction::Constant operand")),
            1 => Instruction::Null,
            2 => Instruction::True,
            3 => Instruction::False,
            4 => Instruction::Pop,
            5 => Instruction::GetLocal(self.read_usize("Instruction::GetLocal operand")),
            6 => Instruction::SetLocal(self.read_usize("Instruction::SetLocal operand")),
            7 => Instruction::GetGlobal(self.read_usize("Instruction::GetGlobal operand")),
            8 => Instruction::DefineGlobal(self.read_usize("Instruction::DefineGlobal operand")),
            9 => Instruction::SetGlobal(self.read_usize("Instruction::SetGlobal operand")),
            10 => Instruction::Print,
            11 => Instruction::Not,
            12 => Instruction::Equal,
            13 => Instruction::Greater,
            14 => Instruction::Less,
            15 => Instruction::Negate,
            16 => Instruction::Add,
            17 => Instruction::Subtract,
            18 => Instruction::Multiply,
            19 => Instruction::Divide,
            20 => Instruction::JumpForward(self.read_usize("Instruction::JumpForward operand")),
            21 => Instruction::JumpForwardIfFalse(
                self.read_usize("Instruction::JumpForwardIfFalse operand"),
            ),
            22 => Instruction::JumpBackward(self.read_usize("Instruction::JumpBackward operand")),
            _ => panic!("Instruction not supported"),
        };
        self.instructions.push(instruction);
    }
    fn read_usize(&mut self, for_what: &str) -> usize {
        expect(for_what, self.reader.read_usize())
    }
    fn read_byte(&mut self, for_what: &str) -> u8 {
        expect(for_what, self.reader.read_byte())
    }
    fn read_bytes(&mut self, n_bytes: usize, for_what: &str) -> Vec<u8> {
        expect(for_what, self.reader.read_bytes(n_bytes))
    }
}

fn expect<T>(what: &str, result: Result<T, usize>) -> T {
    match result {
        Err(i) => panic!(
            "Bytecode ended unexpectedly at position {} expected {}",
            i, what
        ),
        Ok(t) => t,
    }
}

fn integer_from_byte_constant(byte: u8) -> i8 {
    i8::from_le_bytes([byte])
}

#[cfg(test)]
mod tests {
    use crate::bytecode::bytecode_reader::BytecodeReader;
    use crate::bytecode::chunk::{Chunk, ChunkConstant, ChunkConstantOrdinal};
    use crate::bytecode::chunk_parser::integer_from_byte_constant;
    use crate::bytecode::chunk_parser::ChunkParser;
    use crate::bytecode::instructions::Instruction;
    use std::convert::TryFrom;

    fn make_bytes(n_constants: u8, constants: Vec<u8>, code: Vec<u8>) -> Vec<u8> {
        let mut result: Vec<u8> = vec![n_constants];
        result.extend(constants);
        result.extend(code);
        result
    }

    fn make_constants(size: u8) -> (Vec<u8>, Vec<ChunkConstant>) {
        let mut raw_constants: Vec<u8> = vec![];
        let mut chunk_constants: Vec<ChunkConstant> = vec![];
        for i in 0..size {
            raw_constants.push(ChunkConstantOrdinal::Int as u8);
            raw_constants.push(i);
            chunk_constants.push(ChunkConstant::INT(integer_from_byte_constant(i)));
        }
        (raw_constants, chunk_constants)
    }

    fn make_many_instructions(size: u8) -> (Vec<u8>, Vec<Instruction>) {
        let mut raw_instr: Vec<u8> = vec![];
        let mut instr: Vec<Instruction> = vec![];

        for i in 0..size {
            // Const(i)
            instr.push(Instruction::Constant(usize::from(i)));
            raw_instr.push(0);
            raw_instr.push(i);
            // True
            instr.push(Instruction::True);
            raw_instr.push(2);
        }
        (raw_instr, instr)
    }

    fn parse_bytes(bytes: Vec<u8>) -> Chunk {
        let mut reader = BytecodeReader::new(bytes);
        ChunkParser::new(&mut reader).parse()
    }

    #[test]
    fn should_parse_0_constants_and_0_code() {
        let bytes: Vec<u8> = make_bytes(0, vec![], vec![]);
        let chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: vec![],
            code: vec![],
        };
        assert_eq!(expected_chunk, chunk);
    }

    #[test]
    fn should_parse_1_constant_and_0_code() {
        let bytes: Vec<u8> = make_bytes(1, vec![ChunkConstantOrdinal::Int as u8, 50], vec![]);
        let chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: vec![ChunkConstant::INT(50)],
            code: vec![],
        };
        assert_eq!(expected_chunk, chunk);
    }

    #[test]
    fn should_parse_many_constants_and_0_code() {
        let (raw, actual) = make_constants(50);

        let bytes: Vec<u8> = make_bytes(50, raw, vec![]);
        let chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: actual,
            code: vec![],
        };
        assert_eq!(expected_chunk, chunk);
    }

    #[test]
    fn should_parse_0_constants_and_1_instruction_of_arity_0() {
        let actual = vec![Instruction::Add];
        let raw = vec![16 as u8];

        let bytes: Vec<u8> = make_bytes(0, vec![], raw);
        let actual_chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: vec![],
            code: actual,
        };
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_0_constants_and_1_instruction_of_arity_1() {
        let actual = vec![Instruction::Constant(0)];
        let raw = vec![0 as u8, 0 as u8];

        let bytes: Vec<u8> = make_bytes(0, vec![], raw);
        let actual_chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: vec![],
            code: actual,
        };
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_0_constants_and_many_instruction_of_varying_arity() {
        let (raw_instr, actual_instr) = make_many_instructions(50);

        let bytes: Vec<u8> = make_bytes(0, vec![], raw_instr);
        let actual_chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: vec![],
            code: actual_instr,
        };
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_many_constants_and_many_instruction_of_varying_arity() {
        let (raw_const, actual_const) = make_constants(40);
        let (raw_instr, actual_instr) = make_many_instructions(50);

        let bytes: Vec<u8> = make_bytes(40, raw_const, raw_instr);
        let actual_chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: actual_const,
            code: actual_instr,
        };
        assert_eq!(expected_chunk, actual_chunk);
    }

    // Strings

    fn encode_chunk_constant(chunk_constant: ChunkConstant) -> Vec<u8> {
        match chunk_constant {
            ChunkConstant::INT(i) => {
                vec![ChunkConstantOrdinal::Int as u8, u8::try_from(i).unwrap()]
            }
            ChunkConstant::STRING(str) => {
                let raw = str.as_bytes();
                let size = u8::try_from(raw.len()).unwrap();
                let mut result: Vec<u8> = vec![ChunkConstantOrdinal::String as u8, size];
                result.extend(raw);
                result
            }
        }
    }

    fn encode_all_chunk_constants(constants: Vec<ChunkConstant>) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for constant in constants {
            result.extend(encode_chunk_constant(constant))
        }
        result
    }

    #[test]
    fn should_parse_1_string_constant_and_no_instructions() {
        let string = String::from("Hello world");
        let constants = vec![ChunkConstant::STRING(string.clone())];
        let constant_bytes = encode_all_chunk_constants(constants);
        let bytes = make_bytes(1, constant_bytes, vec![]);

        let expected_chunk = Chunk {
            constants: vec![ChunkConstant::STRING(string)],
            code: vec![],
        };
        let actual_chunk = parse_bytes(bytes);
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_2_string_constants_and_no_instructions() {
        let string = String::from("Hello world");
        let string2 = String::from("Another string");
        let constants = vec![
            ChunkConstant::STRING(string.clone()),
            ChunkConstant::STRING(string2.clone()),
        ];
        let constant_bytes = encode_all_chunk_constants(constants);
        let bytes = make_bytes(2, constant_bytes, vec![]);

        let expected_chunk = Chunk {
            constants: vec![
                ChunkConstant::STRING(string),
                ChunkConstant::STRING(string2),
            ],
            code: vec![],
        };
        let actual_chunk = parse_bytes(bytes);
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_int_string_and_int_and_no_instructions() {
        let string = String::from("Hello world");
        let constants = vec![
            ChunkConstant::INT(100),
            ChunkConstant::STRING(string.clone()),
            ChunkConstant::INT(70),
        ];
        let constant_bytes = encode_all_chunk_constants(constants);
        let bytes = make_bytes(3, constant_bytes, vec![]);

        let expected_chunk = Chunk {
            constants: vec![
                ChunkConstant::INT(100),
                ChunkConstant::STRING(string.clone()),
                ChunkConstant::INT(70),
            ],
            code: vec![],
        };
        let actual_chunk = parse_bytes(bytes);
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_string_and_1_instruction() {
        let string = String::from("Hello world");
        let constants = vec![ChunkConstant::STRING(string.clone())];

        let actual_instructions = vec![Instruction::Constant(0)];
        let instructions_raw = vec![0 as u8, 0 as u8];

        let bytes: Vec<u8> = make_bytes(1, encode_all_chunk_constants(constants), instructions_raw);
        let actual_chunk = parse_bytes(bytes);

        let expected_chunk = Chunk {
            constants: vec![ChunkConstant::STRING(string)],
            code: actual_instructions,
        };
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_jump_forward() {
        let actual_instructions = vec![Instruction::JumpForward(1)];
        let instructions_raw = vec![20 as u8, 1 as u8];
        let bytes: Vec<u8> = make_bytes(0, vec![], instructions_raw);

        let expected_chunk = Chunk {
            constants: vec![],
            code: actual_instructions,
        };
        let actual_chunk = parse_bytes(bytes);
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_jump_forward_if_false() {
        let actual_instructions = vec![Instruction::JumpForwardIfFalse(4)];
        let instructions_raw = vec![21 as u8, 4 as u8];
        let bytes: Vec<u8> = make_bytes(0, vec![], instructions_raw);

        let expected_chunk = Chunk {
            constants: vec![],
            code: actual_instructions,
        };
        let actual_chunk = parse_bytes(bytes);
        assert_eq!(expected_chunk, actual_chunk);
    }

    #[test]
    fn should_parse_jump_backward() {
        let actual_instructions = vec![Instruction::JumpBackward(10)];
        let instructions_raw = vec![22 as u8, 10 as u8];
        let bytes: Vec<u8> = make_bytes(0, vec![], instructions_raw);

        let expected_chunk = Chunk {
            constants: vec![],
            code: actual_instructions,
        };
        let actual_chunk = parse_bytes(bytes);
        assert_eq!(expected_chunk, actual_chunk);
    }
}
