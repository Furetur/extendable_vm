use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// A property that splits exceptions into 2 groups
///
/// `Static` exceptions can be detected before code execution.
/// These include errors in the format of bytecode, encoding errors etc.
/// The rest of exceptions are `Runtime` which means that invalid code can lead to such exceptions.
///
#[derive(Debug)]
pub enum ExceptionType {
    Static,
    Runtime,
}

impl Display for ExceptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExceptionType::Runtime => write!(f, "Runtime"),
            ExceptionType::Static => write!(f, "Syntax"),
        }
    }
}

/// An error that is raised by the VM
///
/// Exception contains a type (`exception_type`), a name and a message that are displayed
/// with the stacktrace to the user.
#[derive(Debug)]
pub struct Exception {
    pub exception_type: ExceptionType,
    pub name: String,
    pub message: String,
}

impl Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.exception_type, self.name, self.message
        )
    }
}

impl Error for Exception {}
