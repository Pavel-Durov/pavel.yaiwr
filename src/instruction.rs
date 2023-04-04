use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StackValue {
    Integer(u64),
    Boolean(bool),
}

impl Display for StackValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let a = match self {
            StackValue::Integer(val) => f.write_str(format!("{}", val).as_str()),
            StackValue::Boolean(val) => f.write_str(format!("{}", val).as_str()),
        };
        return a;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Add,
    Mul,
    LessThan,
    GreaterThan,
    Push {
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

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Instruction::Add => f.write_str("Add"),
            Instruction::Mul => f.write_str("Mul"),
            Instruction::LessThan => f.write_str("LessThan"),
            Instruction::Push { value: _ } => f.write_str("Push"),
            Instruction::PrintLn => f.write_str("PrintLn"),
            Instruction::Assign { id: _ } => f.write_str("Assign"),
            Instruction::Load { id: _ } => f.write_str("Load"),
            Instruction::Return { block: _ } => f.write_str("Return"),
            Instruction::Function {
                id: _,
                params: _,
                block: _,
            } => f.write_str("Function"),
            Instruction::FunctionCall { id: _, args: _ } => f.write_str("FunctionCall"),
            Instruction::GreaterThan => f.write_str("GreaterThan"),
        }
    }
}
