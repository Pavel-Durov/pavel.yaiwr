use std::fmt::{Display, Error, Formatter};

use crate::err::InterpError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StackValue {
    Integer(u64),
    Boolean(bool),
}
impl StackValue {
    pub fn as_int(&self) -> Result<u64, InterpError> {
        match self {
            StackValue::Integer(v) => Ok(*v),
            a => Err(InterpError::EvalError(
                format!("Expected StackValue Integer stack, got {}!", a).to_string(),
            )),
        }
    }
    pub fn as_bool(&self) -> Result<bool, InterpError> {
        match self {
            StackValue::Boolean(v) => Ok(*v),
            a => Err(InterpError::EvalError(
                format!("Expected StackValue Boolean, got {}!", a).to_string(),
            )),
        }
    }
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
pub enum BinaryOp {
    LessThan,
    GreaterThan,
    Add,
    Mul,
    Assign { id: String },
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            BinaryOp::LessThan => f.write_str("LessThan"),
            BinaryOp::GreaterThan => f.write_str("GreaterThan"),
            BinaryOp::Add => f.write_str("Add"),
            BinaryOp::Mul => f.write_str("Mul"),
            BinaryOp::Assign { id: _ } => f.write_str("Assign"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    BinaryOp {
        op: BinaryOp,
    },
    Push {
        value: StackValue,
    },
    PrintLn,
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
            Instruction::Push { value: _ } => f.write_str("Push"),
            Instruction::PrintLn => f.write_str("PrintLn"),
            Instruction::Load { id: _ } => f.write_str("Load"),
            Instruction::Return { block: _ } => f.write_str("Return"),
            Instruction::Function {
                id: _,
                params: _,
                block: _,
            } => f.write_str("Function"),
            Instruction::FunctionCall { id: _, args: _ } => f.write_str("FunctionCall"),
            Instruction::BinaryOp { op } => f.write_str(format!("BinaryOp({})", op).as_str()),
        }
    }
}
