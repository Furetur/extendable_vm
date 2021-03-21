use jex_vm::chunk::{Chunk, ChunkConstant, Instruction};
use jex_vm::vm::VM;

#[test]
fn null_should_not_be_equal_to_int() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::INT(1)],
        code: vec![
            Instruction::Constant(0),
            Instruction::Null,
            Instruction::Equal,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(!result.unwrap().as_bool())
}

#[test]
fn null_should_not_be_equal_to_bool() {
    let chunk = Chunk {
        constants: vec![],
        code: vec![Instruction::False, Instruction::Null, Instruction::Equal],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(!result.unwrap().as_bool())
}

#[test]
fn null_should_be_equal_to_null() {
    let chunk = Chunk {
        constants: vec![],
        code: vec![Instruction::Null, Instruction::Null, Instruction::Equal],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.unwrap().as_bool())
}

// TODO: should add tests that should panic when anything is done with null
