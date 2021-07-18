use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

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
