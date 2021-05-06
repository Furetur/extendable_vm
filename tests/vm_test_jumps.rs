// use jex_vm::code::chunk::{Chunk, ChunkConstant};
// use jex_vm::jex::instructions::Instruction;
// use jex_vm::runtime::vm::VM;
//
// #[test]
// fn should_exit_if_jumped_too_far() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::JumpForward(2), Instruction::Constant(0)],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert!(result.is_none())
// }
//
// #[test]
// fn should_skip_one_instruction_by_jumping() {
//     let chunk = Chunk {
//         constants: vec![ChunkConstant::INT(0), ChunkConstant::INT(1)],
//         code: vec![
//             Instruction::Constant(0),
//             Instruction::JumpForward(1),
//             Instruction::Constant(1),
//         ],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert_eq!(0, result.unwrap().as_int())
// }
//
// #[test]
// fn should_skip_2_instructions_by_jumping() {
//     let chunk = Chunk {
//         constants: vec![ChunkConstant::INT(0), ChunkConstant::INT(1)],
//         code: vec![
//             Instruction::Constant(0),
//             Instruction::JumpForward(2),
//             Instruction::Constant(0),
//             Instruction::Constant(1),
//         ],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert_eq!(0, result.unwrap().as_int())
// }
//
// #[test]
// #[should_panic]
// fn should_panic_if_jumped_before_code() {
//     let chunk = Chunk {
//         constants: vec![],
//         code: vec![Instruction::Constant(0), Instruction::JumpBackward(2)],
//     };
//     let mut vm = VM::new();
//     vm.run(&chunk);
// }
//
// #[test]
// fn should_skip_one_instruction_by_jumping_if_false() {
//     let chunk = Chunk {
//         constants: vec![ChunkConstant::INT(0), ChunkConstant::INT(1)],
//         code: vec![
//             Instruction::Constant(0),
//             Instruction::False,
//             Instruction::JumpForwardIfFalse(1),
//             Instruction::Constant(1),
//             Instruction::Pop
//         ],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert_eq!(0, result.unwrap().as_int())
// }
//
// #[test]
// fn should_not_skip_one_instruction_by_jumping_if_true() {
//     let chunk = Chunk {
//         constants: vec![ChunkConstant::INT(0), ChunkConstant::INT(1)],
//         code: vec![
//             Instruction::Constant(0),
//             Instruction::True,
//             Instruction::JumpForwardIfFalse(1),
//             Instruction::Constant(1),
//         ],
//     };
//     let mut vm = VM::new();
//     let result = vm.run(&chunk);
//     assert_eq!(1, result.unwrap().as_int())
// }
