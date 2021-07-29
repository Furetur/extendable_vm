# Extendable VM

> Simplifies writing stack virtual machines in Rust

Just define your:
* bytecode format
* instructions

and then run the VM!

This was originally a part of [jex_vm](https://github.com/Furetur/jex_vm),
a stack VM for my simple programming language [Jex](https://github.com/Furetur/JexCompiler).

## Getting Started

### Installing

Just add extendable_vm to `Cargo.toml`:

```toml
[dependencies]
extendable_vm = "<latest version>"
```

You can get the latest version from the [Releases page](https://github.com/Furetur/extendable_vm/releases).

### Run with logging

If you are using extendable_vm in your binary executable and wish to view all VM logs
then add `extendable_vm` to `RUST_LOG` environment variable: `RUST_LOG=extendable_vm`.
If your environment variable already defines a list of options (`RUST_LOG=a,b,c`)
then just append extendable_vm: `RUST_LOG=a,b,c,extendable_vm`

For example,

```shell
RUST_LOG=extendable_vm ./your_binary_exec path/to/bytecode
```

### Basic Concepts

The virtual machine reads _Code_ which consists of several independent parts called _Chunks_, which contain executable code and _constants_ (such as `1`, `2`, or `"Hello World"`). The VM has an operand stack, a call stack and can jump inside one chunk or between chunks.

Executable code is just an array of bytes that encodes a list of instructions that should be run. Each instruction has its unique id -- _opcode_ and a number of arguments that it accepts.

For example, if instruction `A` with _opcode_ = 7 accepts 2 arguments then we can run `7 1 2 7 3 4` which means `run A(1, 2); run A(3, 4)`.

To construct your own VM you must define:

* types of _constants_ that are be stored in your bytecode and how they should be parsed. [More about bytecode format](#Bytecode).
* the set of _instructions_ that your VM can execute and implement them. [More about instructions](#defining-instructions).

### VM state

State of the VM is represented by a `Machine<Constant, Value>` struct. It stores:
* code that the VM is executing
* stack of operands
* call stack
* global values

`Constant` is the type of the constant values in bytecode.

`Value`s are operands that the VM manipulates.

### Defining instructions

Each instruction has its unique ID -- `op_code`, `name` that is used for debugging.
And a function `instruction_fn` that implements the logic of the instruction.

```rust
pub struct Instruction<Constant, Value> {
    pub op_code: u8,
    pub name: &'static str,
    pub instruction_fn: InstructionFn<Constant, Value>,
}
```

`InstructionFn` can be interpreted as a simple function that accepts the state of the VM
and a list of arguments that the instruction receives and mutates the VM state.
But it also has several features that simplify defining new instructions.
`Const`, `UnaryOp` and `BinaryOp` simplify the creation on nullary, unary and binary operator instructions respectively.

```rust
pub enum InstructionFn<Constant, Value> {
    // Simple function that I described above
    Raw {
        byte_arity: usize,
        instruction_fn: RawInstructionFn<Constant, Value>,
    },
    // Instruction that generates a value and pushes it onto the stack
    Const(fn() -> Value),
    // Unary operator instruction that pops the value from stack,
    // produces new value and pushes it onto the stack
    UnaryOp(fn(value: Value) -> Result<Value, Exception>),
    // The same as unary operator but pops 2 values
    BinaryOp(fn(left: Value, right: Value) -> Result<Value, Exception>),
}

// Simple function that I described above
// (mut VM State, instruction arguments) -> may return Exception
pub type RawInstructionFn<Constant, Value> = fn(
    machine: &mut Machine<Constant, Value>,
    args_ip: InstructionPointer,
) -> Result<(), Exception>;
```

### Bytecode

This section describes how bytecode can be accessed in API and how it is represented in a binary file.

#### Notation for binary files

In the context of binary data `struct`s are used as a way to demonstrate what each byte means. 
Each struct in this context should be viewed as an array of bytes
where each value directly follow the previous (without padding and packing).

For example, struct `A` represents bytes `a1 a2 b` where `a1` and `a2` correspond to `a: u16` and `b` to `b: u8`.

```rust
struct A {
    a: u16,
    b: u8
}
```

#### Code

Virtual machine reads `Code` (bytecode) and executes it. `Code` consists of several independent executable pieces -- `Chunk`s. For instance, each function should be defined as a separate `Chunk`.

```rust
// API
pub struct Code<Constant> {
    pub chunks: Vec<Chunk<Constant>>,
}

// in binary file
struct _Code<Constant> {
    chunks: [_Chunk<Constant>]
}
```

In a binary file `Code` is represented as an array of bytes where all chunks are concatenated. For example, if `chunk1` is represented by bytes `00 01` and `chunks2` -- `02 03`. Then code `[chunk1, chunk2]` is `00 01 02 03`.

#### Chunk

Each `Chunk` has several `constants` and executable `code` which is just an array of bytes.

```rust
// API
pub struct Chunk<Constant> {
    pub constants: Vec<Constant>,
    pub code: Vec<u8>,
}

// in binary file
struct _Chunk<Constant> {
    // number of constants
    n_constants: u8,
    // array of constants of size `n_constants`
    // each constant is encoded as an array of bytes and is parsed by a constant parser
    constants: [Constant],
    // number of bytes in `code`
    n_code_bytes: u16,
    // executable code
    code: [u8]
}
```

#### Parsing code

`CodeParser` and `ConstantParser` are useful abstractions that simplify parsing bytecode.
However, using them is not necessary and you may create a `Code` struct in any way you want.

`CodeParser` assumes that all chunk constants are represented in a binary file by a unique id and an array of bytes.
Each type of constants should be parsed by a separate `ConstantParser`.

For example, if we have `IntConstant` that holds `i32` we can define a parser:

```rust
// in binary file
struct _IntConstant {
    // unique ID = 0
    constant_type: 0 as u8, // used only to demonstrate binary data
    // 4 bytes that represent i32
    data: [u8]
}

const INT_CONSTANT_PARSER: ConstantParser<i32> = ConstantParser {
    constant_type: 0 as u8,
    parser_fn: parse_int_constant,
};

// parses `data` and returns i32 or on exception
fn parse_int_constant(
    // the entire code
    bytes: &RawBytes,
    // points to the current reading position in `bytes`
    // initially points to the start of `data`
    pointer: &mut RawBytesPointer, 
) -> Result<i32, Exception> {
    // all read operations advance the `pointer`
    Ok(bytes.read_i32(pointer).unwrap())
}
```

## Building from source

### Build a development version

```shell
cargo build
```

### Build a release version

```shell
cargo build --release
```

### Run tests

```shell
cargo test
```

## History

I wanted to learn about compilers and programming languages
and ended up reading this great book [Crafting Interpreters](https://craftinginterpreters.com/contents.html)
and making my programming language [Jex](https://github.com/Furetur/JexCompiler).

This was originally a part of a simple VM for my programming language [jex_vm](https://github.com/Furetur/jex_vm),
my first Rust project.

The design of this library is inspired by [stack_vm](https://docs.rs/stack-vm/1.0.1/stack_vm/)
which helped a lot since I did not know anything about Rust before working on this project.
