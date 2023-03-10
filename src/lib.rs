use instruction::Instruction;
use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};
use std::collections::HashMap;

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub mod ast;
pub mod err;
pub mod instruction;

use ast::AstNode;
use err::InterpError;

pub struct Calc {
    var_store: HashMap<String, u64>,
    stack: Vec<u64>,
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            var_store: HashMap::new(),
            stack: vec![],
        }
    }

    pub fn get_var(&self, id: String) -> Result<&u64, InterpError> {
        return self
            .var_store
            .get(&id)
            .ok_or(InterpError::VariableNotFound(id));
    }

    pub fn stack_pop(&mut self) -> Result<u64, InterpError> {
        return self.stack.pop().ok_or(InterpError::EmptyStack);
    }

    pub fn stack_push(&mut self, val: u64) {
        self.stack.push(val);
    }

    pub fn from_str(&self, input: &str) -> Result<AstNode, InterpError> {
        let lexer_def = calc_l::lexerdef();
        let lexer = lexer_def.lexer(input);
        let (ast_exp, errs) = calc_y::parse(&lexer);

        let err_msg = self.get_parse_err(&lexer, errs);
        if err_msg.is_empty() == false {
            return Err(InterpError::ParseError(err_msg));
        }

        match ast_exp {
            Some(res) => match res {
                Ok(exp) => Ok(exp),
                Err(_) => Err(InterpError::ParseError(err_msg)),
            },
            None => Err(InterpError::ParseError(err_msg)),
        }
    }

    fn get_parse_err(
        &self,
        lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
        errs: Vec<LexParseError<u32, DefaultLexerTypes>>,
    ) -> String {
        let msgs = errs
            .iter()
            .map(|e| e.pp(lexer, &calc_y::token_epp))
            .collect::<Vec<String>>();
        return msgs.join("\n");
    }

    pub fn to_bytecode(&self, ast_node: AstNode, prog: &mut Vec<Instruction>) {
        match ast_node {
            AstNode::Add { lhs, rhs } => {
                self.to_bytecode(*lhs, prog);
                self.to_bytecode(*rhs, prog);
                prog.push(Instruction::Add {})
            }
            AstNode::Mul { lhs, rhs } => {
                self.to_bytecode(*lhs, prog);
                self.to_bytecode(*rhs, prog);
                prog.push(Instruction::Mul {})
            }
            AstNode::Number { value } => prog.push(Instruction::Push { value: value }),
            AstNode::PrintLn { rhs } => {
                self.to_bytecode(*rhs, prog);
                prog.push(Instruction::PrintLn {})
            }
            AstNode::Assign { id, rhs } => {
                self.to_bytecode(*rhs, prog);
                prog.push(Instruction::Assign { id })
            }
            AstNode::ID { value } => prog.push(Instruction::Load { id: value }),
        }
    }
    pub fn eval(&mut self, instructions: &Vec<Instruction>) -> Result<Option<u64>, InterpError> {
        for instruction in instructions {
            match instruction {
                Instruction::Push { value } => self.stack.push(*value),
                Instruction::PrintLn {} => {
                    println!("{}", self.stack.pop().unwrap())
                }
                Instruction::Mul {} => {
                    let val = self
                        .stack_pop()?
                        .checked_mul(self.stack_pop()?)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?;
                    self.stack_push(val);
                }
                Instruction::Add {} => {
                    let val = self
                        .stack_pop()?
                        .checked_add(self.stack_pop()?)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?;
                    self.stack_push(val);
                }
                Instruction::Assign { id } => {
                    let val = self.stack_pop()?;
                    self.var_store.insert(id.to_string(), val);
                }
                Instruction::Load { id } => {
                    self.stack_push(*self.get_var(id.into())?);
                }
            }
        }
        if self.stack.is_empty() {
            return Ok(None);
        }
        return Ok(Some(self.stack.pop().unwrap()));
    }
}
