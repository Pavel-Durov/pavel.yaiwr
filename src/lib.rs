use bytecode::block_to_bytecode;
use instruction::{BinaryOp, EvalResult, Instruction, StackValue};
use log::debug;
use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};
use scope::Scope;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub mod ast;
pub mod bytecode;
pub mod err;
pub mod instruction;
pub mod scope;

use ast::AstNode;
use err::InterpError;

use crate::instruction::JumpInstruction;

pub struct Calc {
    fun_store: HashMap<String, Instruction>,
    stack: Vec<StackValue>,
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            fun_store: HashMap::new(),
            stack: vec![],
        }
    }

    pub fn stack_pop(&mut self) -> Result<StackValue, InterpError> {
        return Ok(self.stack.pop().ok_or(InterpError::EmptyStack)?);
    }

    pub fn stack_push(&mut self, val: StackValue) {
        self.stack.push(val);
    }

    pub fn from_str(&self, input: &str) -> Result<Vec<AstNode>, InterpError> {
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
                Err(a) => {
                    println!("{:?}", a);
                    Err(InterpError::ParseError(err_msg))
                },
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

    pub fn ast_to_bytecode(ast: Vec<AstNode>) -> Vec<Instruction> {
        return block_to_bytecode(ast);
    }

    fn eval_function_args(
        &mut self,
        args: &Vec<Vec<Instruction>>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Vec<EvalResult>, InterpError> {
        let mut result = vec![];
        for arg_set in args {
            match self.eval(arg_set, scope.clone()) {
                Ok(Some(x)) => result.push(x),
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }
        return Ok(result);
    }

    fn eval_function_call<'a>(
        &mut self,
        args: &Vec<EvalResult>,
        id: &String,
        outer_scope: Rc<RefCell<Scope>>,
    ) -> Result<Option<EvalResult>, InterpError> {
        let function = self
            .fun_store
            .get(id)
            .ok_or(InterpError::UndefinedFunction(id.to_string()))?;
        match function {
            Instruction::Function {
                id: _,
                params,
                block: body,
            } => {
                if params.len() != args.len() {
                    return Err(InterpError::EvalError(format!(
                        "Unexpected number of function arguments. Expected: {}, Got: {}",
                        params.len(),
                        args.len()
                    )));
                }

                let mut func_scope = Scope::from_scope(outer_scope);
                // bind args and params to funciton scope
                for (i, arg) in args.iter().enumerate() {
                    if let EvalResult::Value(val) = arg {
                        func_scope.dec_var(params[i].clone());
                        func_scope.set_var(params[i].clone(), *val)?;
                    }
                }

                return self.eval(&body.clone(), Rc::new(RefCell::new(func_scope)));
            }
            _ => {
                return Err(InterpError::EvalError(
                    "Unexpected type registrated as a function!".to_string(),
                ));
            }
        }
    }

    fn eval_binary_op(
        &mut self,
        op: &BinaryOp,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Option<StackValue>, InterpError> {
        let val = match op {
            BinaryOp::LessThan => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                StackValue::Boolean(op2 < op1)
            }
            BinaryOp::GreaterThan => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                StackValue::Boolean(op1 < op2)
            }
            BinaryOp::Add => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                StackValue::Integer(
                    op1.checked_add(op2)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?,
                )
            }
            BinaryOp::Mul => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                StackValue::Integer(
                    op1.checked_mul(op2)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?,
                )
            }
            BinaryOp::Assign { id } => {
                let val = self.stack_pop()?;
                scope.borrow_mut().set_var(id.to_string(), val)?;
                val
            }
            BinaryOp::Declare { id } => {
                scope.borrow_mut().dec_var(id.to_string());
                StackValue::Uninitialised
            }
            BinaryOp::Equal => {
                let val = self.eval_eq()?;
                self.stack.push(val);
                val
            }
            BinaryOp::NotEqual => {
                let val = StackValue::Boolean(!self.eval_eq()?.as_bool()?);
                self.stack.push(val);
                val
            }
            BinaryOp::LogicalAnd => {
                let op1 = self.stack_pop()?;
                let op2 = self.stack_pop()?;
                let stack_value;
                if op1.is_same_type(&op2) {
                    stack_value = StackValue::Boolean(op1.as_bool()? && op2.as_bool()?);
                    self.stack.push(stack_value)
                } else {
                    return Err(InterpError::EvalError(
                        format!(
                            "Operand {} and Operand {} cannot be applied to logical LogicalAnd operation",
                            op1, op2
                        )
                        .to_string(),
                    ));
                }
                stack_value
            }
            BinaryOp::LogicalOr => {
                let op1 = self.stack_pop()?;
                let op2 = self.stack_pop()?;
                let stack_value;
                if op1.is_same_type(&op2) {
                    stack_value = StackValue::Boolean(op1.as_bool()? || op2.as_bool()?);
                } else {
                    return Err(InterpError::EvalError(
                        format!(
                            "Operand {} and Operand {} cannot be applied to logical LogicalOr operation",
                            op1, op2
                        )
                        .to_string(),
                    ));
                }
                stack_value
            }
        };
        self.stack_push(val);
        Ok(Some(val))
    }

    fn eval_eq(&mut self) -> Result<StackValue, InterpError> {
        let op1 = self.stack_pop()?;
        let op2 = self.stack_pop()?;
        let stack_value;
        if op1.is_same_type(&op2) {
            stack_value = StackValue::Boolean(op1 == op2);
            self.stack.push(stack_value)
        } else {
            return Err(InterpError::EvalError(
                format!(
                    "Operand {} and Operand {} cannot be applied to logical LogicalOr operation",
                    op1, op2
                )
                .to_string(),
            ));
        }
        Ok(stack_value)
    }

    pub fn eval_input(input: String) -> Result<Option<EvalResult>, InterpError> {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let calc = &mut Calc::new();
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, scope)
    }

    pub fn eval(
        &mut self,
        instructions: &Vec<Instruction>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Option<EvalResult>, InterpError> {
        for instruction in instructions {
            debug!("eval: {:?}. scope: {:?}", instruction, scope.clone());

            match instruction {
                Instruction::Return { block } => {
                    let val = self.eval(block, scope.clone())?;
                    if let Some(EvalResult::Value(v)) = val {
                        self.stack_push(v);
                    }
                    return Ok(Some(EvalResult::Jump(JumpInstruction::Return {})));
                }
                Instruction::Function {
                    block: body,
                    id,
                    params,
                } => {
                    if let None = self.fun_store.get(id) {
                        self.fun_store.insert(
                            id.to_string(),
                            Instruction::Function {
                                id: id.to_string(),
                                params: params.to_vec(),
                                block: body.to_vec(),
                            },
                        );
                    } else {
                        return Err(InterpError::EvalError(format!(
                            "Function with the id: '{}' already defined!",
                            id
                        )));
                    }
                }
                Instruction::FunctionCall { id, args } => {
                    let arg_list = self.eval_function_args(&args, scope.clone())?;
                    let res = self.eval_function_call(&arg_list, id, scope.clone())?;
                    if let Some(EvalResult::Value(x)) = res {
                        self.stack_push(x);
                    }
                }
                Instruction::Push { value } => self.stack.push(*value),
                Instruction::PrintLn => {
                    println!("{}", self.stack_pop()?);
                }
                Instruction::Load { id } => {
                    let val = scope.borrow().get_var(id.to_string())?;
                    self.stack_push(val);
                }
                Instruction::BinaryOp { op } => {
                    self.eval_binary_op(op, scope.clone())?;
                }
                Instruction::Conditional {
                    condition,
                    block,
                    alternative,
                } => {
                    if let Ok(Some(EvalResult::Value(StackValue::Boolean(val)))) =
                        self.eval(condition, scope.clone())
                    {
                        let mut block_result = None;
                        if val {
                            block_result = self.eval(block, scope.clone())?;
                        } else if let Some(alt) = alternative {
                            block_result = self.eval(alt, scope.clone())?;
                        }
                        if let Some(EvalResult::Jump(JumpInstruction::Return)) = block_result {
                            break;
                        }
                    }
                }
            }
        }
        let result;
        if self.stack.is_empty() {
            result = Ok(None);
        } else {
            let val = self.stack_pop()?;
            result = Ok(Some(EvalResult::Value(val)));
        }
        debug!("eval:result {:?}", &result);
        return result;
    }
}
