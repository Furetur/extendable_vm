pub trait RuntimeError {
    fn get_type(&self) -> &str;
    fn get_message(&self) -> &String;

    fn to_output_string(&self) -> String {
        format!("{}: {}", self.get_type(), self.get_message())
    }
}

pub struct CodeReadingError(String);

impl RuntimeError for CodeReadingError {
    fn get_type(&self) -> &str {
        "CodeReadingError"
    }
    fn get_message(&self) -> &String {
        let CodeReadingError(message) = self;
        message
    }
}

pub struct TypeError(String);

impl RuntimeError for TypeError {
    fn get_type(&self) -> &str {
        "TypeError"
    }
    fn get_message(&self) -> &String {
        let TypeError(message) = self;
        message
    }
}

pub struct StackError(String);

impl RuntimeError for StackError {
    fn get_type(&self) -> &str {
        "StackError"
    }
    fn get_message(&self) -> &String {
        let StackError(message) = self;
        message
    }
}
