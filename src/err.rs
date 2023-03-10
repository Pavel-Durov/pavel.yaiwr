use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum InterpError {
    ParseError(String),
    EmptyStack,
    Numeric(String),
    VariableNotFound(String),
    ProgramFileNotFound(String),
}

impl Display for InterpError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InterpError::EmptyStack => f.write_str("Cannot pop from empty stack!"),
            InterpError::ParseError(line) => {
                f.write_str(format!("Parse error: {}!", line).as_str())
            }
            InterpError::Numeric(msg) => f.write_str(format!("Numeric error: {}!", msg).as_str()),
            InterpError::VariableNotFound(id) => {
                f.write_str(format!("Variable with id '{}' cannot be found!", id).as_str())
            }
            InterpError::ProgramFileNotFound(file_name) => {
                f.write_str(format!("Program file: '{}' cannot be found!", file_name).as_str())
            }
        }
    }
}

impl Error for InterpError {
    fn description(&self) -> &str {
        match self {
            InterpError::EmptyStack => "EmptyStack",
            InterpError::ParseError(_) => "ParseError",
            InterpError::Numeric(_) => "Numeric",
            InterpError::VariableNotFound(_) => "VariableNotFound",
            InterpError::ProgramFileNotFound(_) => "ProgramFileNotFound",
        }
    }
}
