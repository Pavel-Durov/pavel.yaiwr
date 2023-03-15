use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum InterpError {
    ParseError(String),
    EmptyStack,
    Numeric(String),
    VariableNotFound(String),
    ProgramFileNotFound(String),
    UndefinedFunction(String),
    EvalError(String),
}

impl Display for InterpError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InterpError::EvalError(msg) => {
                f.write_str(format!("Evaluation error: {}!", msg).as_str())
            }
            InterpError::EmptyStack => f.write_str("Cannot pop from empty stack!"),
            InterpError::UndefinedFunction(id) => {
                f.write_str(format!("Cannot find function wiht id '{}'!", id).as_str())
            }
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
            InterpError::EvalError(..) => "EvalError",
            InterpError::UndefinedFunction(..) => "UndefinedFunction",
            InterpError::EmptyStack => "EmptyStack",
            InterpError::ParseError(_) => "ParseError",
            InterpError::Numeric(_) => "Numeric",
            InterpError::VariableNotFound(_) => "VariableNotFound",
            InterpError::ProgramFileNotFound(_) => "ProgramFileNotFound",
        }
    }
}
