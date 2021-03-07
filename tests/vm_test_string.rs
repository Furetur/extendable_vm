use jex_vm::chunk::{Chunk, Instruction, ChunkConstant};
use jex_vm::vm::VM;

#[test]
fn it_concats_two_strings() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::from_str("ab"), ChunkConstant::from_str("cd")],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Add,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert_eq!("abcd", result.unwrap().as_str())
}

#[test]
fn lexically_equal_strings_should_be_equal() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::from_str("ab"), ChunkConstant::from_str("ab")],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Equal,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(result.unwrap().as_bool())
}

#[test]
fn lexically_different_strings_should_not_be_equal() {
    let chunk = Chunk {
        constants: vec![ChunkConstant::from_str("ab"), ChunkConstant::from_str("abc")],
        code: vec![
            Instruction::Constant(0),
            Instruction::Constant(1),
            Instruction::Equal,
        ],
    };
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    assert!(!result.unwrap().as_bool())
}

// TODO: should add panic tests
