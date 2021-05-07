use jex_vm::jex::bytecode_constants::JexConstant;
use jex_vm::jex::instructions::op_codes::JexOpCode;
use jex_vm::machine::code::Chunk;

pub struct TestChunk {
    pub constants: Vec<JexConstant>,
    pub instructions: Vec<TestInstruction>,
}

impl TestChunk {
    pub fn compile(&self) -> Chunk<JexConstant> {
        let mut code: Vec<u8> = vec![];
        for instruction in &self.instructions {
            instruction.compile(&mut code);
        }
        Chunk {
            constants: self.constants.clone(),
            code,
        }
    }
}

pub struct TestInstruction {
    pub op_code: JexOpCode,
    pub args: Vec<u8>,
}

impl TestInstruction {
    pub fn new(op_code: JexOpCode) -> TestInstruction {
        TestInstruction {
            op_code,
            args: vec![],
        }
    }
    pub fn compile(&self, code: &mut Vec<u8>) {
        code.push(self.op_code as u8);
        for arg in &self.args {
            code.push(*arg);
        }
    }
}
