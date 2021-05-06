// use jex_vm::code::chunk::{Chunk, ChunkConstant};
// use jex_vm::jex::instructions::Instruction;
// use jex_vm::runtime::vm::VM;
//
// #[test]
// fn true_and_true_should_be_equal() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::True, Instruction::True, Instruction::Equal],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert!(result.unwrap().as_bool())
// }
//
// #[test]
// fn false_and_false_should_be_equal() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::False, Instruction::Equal],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert!(result.unwrap().as_bool())
// }
//
// #[test]
// fn true_and_false_should_be_equal() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::True, Instruction::False, Instruction::Equal],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert!(!result.unwrap().as_bool())
// }
//
// #[test]
// fn not_true_should_be_false() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::True, Instruction::Not],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert!(!result.unwrap().as_bool())
// }
//
// #[test]
// fn not_false_should_be_false() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::Not],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert!(result.unwrap().as_bool())
// }
//
// // Temporary tests
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bools_are_added() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::True, Instruction::Add],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bools_are_subtracted() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::True, Instruction::Subtract],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bools_are_multiplied() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::True, Instruction::Multiply],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bools_are_divided() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::True, Instruction::Divide],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bools_are_compared_with_greater() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::True, Instruction::Greater],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bools_are_compared_with_less() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::True, Instruction::Less],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// #[should_panic]
// fn it_should_panic_if_bool_is_negated() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::False, Instruction::Negate],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
