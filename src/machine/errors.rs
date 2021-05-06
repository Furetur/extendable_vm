use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct MachineError(pub String);

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MachineError: {}", self.0)
    }
}

impl Error for MachineError {}

//
// #[derive(Debug, Clone)]
// pub struct CodeReadingError(pub String);
//
// impl fmt::Display for CodeReadingError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "CodeReadingError: {}", self.0)
//     }
// }
//
// impl Error for CodeReadingError {}
//
// #[derive(Debug, Clone)]
// pub struct TypeError(pub String);
//
// impl fmt::Display for TypeError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "TypeError: {}", self.0)
//     }
// }
//
// impl Error for TypeError {}
//
// #[derive(Debug, Clone)]
// pub struct StackError(pub String);
//
// impl fmt::Display for StackError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "StackError: {}", self.0)
//     }
// }
//
// impl Error for StackError {}
