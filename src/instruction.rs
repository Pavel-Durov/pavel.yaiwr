use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StackValue {
    Integer(u64),
    Boolean(bool),
}

impl Display for StackValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let a = match self {
            StackValue::Integer(val) => {
                f.write_str(format!("{}", val).as_str())
            },
            StackValue::Boolean(val) => {
                f.write_str(format!("{}", val).as_str())
            }
        };
        return  a;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Add,
    Mul,
    Push{
        value: StackValue,
    },
    PrintLn,
    Assign {
        id: String,
    },
    Load {
        id: String,
    },
    Return {
        block: Vec<Instruction>,
    },
    Function {
        id: String,
        params: Vec<String>,
        block: Vec<Instruction>,
    },
    FunctionCall {
        id: String,
        args: Vec<Vec<Instruction>>,
    },
}
