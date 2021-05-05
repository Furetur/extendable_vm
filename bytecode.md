# Bytecode specs

This file exists because I don't know how to structure my Rust code properly.

**This structs describe how raw bytecode should look.**

### Bytecode
```rust
struct Bytecode {
    n_chunks: u8,
    chunks: [BytecodeChunk]
}
struct BytecodeChunk {
    // constant pool
    n_constants: u8,
    constants: [Constant],
    function_name: u8, // if index is out of bounds, it is global scope
    arity: u8, // should be 0 for global scope
    n_instructions: u32,
    instructions: [Instruction]
}
```

### Constants

Each constant has its type and its data (raw bytes).
Two types of constants are currently supported
```rust
struct IntConstant {
    constant_type: 0,
    value: u8, // will be converted to i8
}
struct StringConstant {
    constant_type: 1,
    string_size: u8,
    utf8_raw_bytes: [u8],
}
struct FunctionConstant {
    constant_type: 2,
    chunk_id: u8, // function's chunk id in the Bytecode.chunks. Starts from 0, but the 0th chunk must be the global script.
}
```
```
type Constant = IntConstant | StringConstant
```

### Instructions

```rust
struct Instruction {
    instruction_type: u8,
    operands: [u8],
}
```

Read about instructions in the source code.
