#[derive(Debug, PartialEq)]
pub enum Instruction {
    // constant instructions and literal instructions
    Constant(usize), // 0
    Null,            // 1
    True,            // 2
    False,           // 3
    // pop from stack
    Pop, // 4
    // variables
    GetLocal(usize),     // 5
    SetLocal(usize),     // 6
    GetGlobal(usize),    // 7
    DefineGlobal(usize), // 8
    SetGlobal(usize),    // 9
    // builtin
    Print, // 10
    // logic operators
    Not, // 11
    // comparison operators
    Equal,   // 12
    Greater, // 13
    Less,    // 14
    // arithmetic operators
    Negate,                    // 15
    Add,                       // 16
    Subtract,                  // 17
    Multiply,                  // 18
    Divide,                    // 19
    JumpForward(usize),        // 20
    JumpForwardIfFalse(usize), // 21
    JumpBackward(usize),       // 22
}
