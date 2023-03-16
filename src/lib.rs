use bytecode::to_bytecode;
use instruction::Instruction;
use log::debug;
use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};
use std::collections::{HashMap};

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub mod ast;
pub mod bytecode;
pub mod err;
pub mod instruction;

use ast::AstNode;
use err::InterpError;

#[derive(Debug)]
pub enum StackValue 
{
    Value(u64),
    Variable(String, u64)
}
#[derive(Debug)]
pub struct Calc {
    // var_store: HashMap<String, u64>,
    fun_store: HashMap<String, Instruction>,
    stack: Vec<StackValue>,
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            // var_store: HashMap::new(),
            fun_store: HashMap::new(),
            stack: vec![],
        }
    }

    pub fn get_var(&self, get_id: String) -> Result<StackValue, InterpError> {
        for var in self.stack.iter() {
            if let StackValue::Variable(id, val) = var {
                if  *id == get_id{
                    return Ok(StackValue::Variable(get_id, *val));
                }
            }
        }
        return Err(InterpError::VariableNotFound(get_id));
    }

    pub fn set_var(&mut self, val: StackValue) {
        self.stack.push(val);
    }

    pub fn stack_pop(&mut self) -> Result<StackValue, InterpError> {
        let val = self.stack.pop().ok_or(InterpError::EmptyStack)?;
        debug!("STACK POP: {:?}", &self.stack);
        return Ok(val);
    }

    pub fn stack_push(&mut self, val: StackValue) {
        self.stack.push(val);
        debug!("STACK PUSH: {:?}", &self.stack);
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
    pub fn ast_to_bytecode(ast: AstNode) -> Vec<Instruction> {
        let bytecode = &mut vec![];
        to_bytecode(ast, bytecode);
        bytecode.to_vec()
    }

    fn eval_function_args(
        &mut self,
        args: &Vec<Vec<Instruction>>,
    ) -> Result<Vec<u64>, InterpError> {
        let mut result = vec![];
        for arg_set in args {
            match self.eval(arg_set) {
                Ok(Some(x)) => result.push(x),
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }
        return Ok(result);
    }

    fn eval_function_call(
        &mut self,
        args: &Vec<u64>,
        id: &String,
    ) -> Result<Option<u64>, InterpError> {
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
                // TODO: Implement function scope/stack based variables
                for (i, p) in params.iter().enumerate() {
                    // self.var_store.insert(p.to_string(), args[i]);
                    let var = StackValue::Variable(p.to_string(), args[i]);
                    // self.set_var(StackValue::Variable(p.to_string(), args[i]))
                    self.stack.push(var);
                }
                return self.eval(&body.clone());
            }
            _ => {
                return Err(InterpError::EvalError(
                    "Unexpected type registrated as a function!".to_string(),
                ));
            }
        }
    }

    pub fn eval(&mut self, instructions: &Vec<Instruction>) -> Result<Option<u64>, InterpError> {
        for instruction in instructions {
            debug!("eval: {:?}", instruction);
            match instruction {
                Instruction::Return { block } => {
                    let val = self.eval(block)?;
                    if let Some(x) = val {
                        self.stack_push(StackValue::Value(x));
                    }
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
                    let arg_list = self.eval_function_args(&args)?;
                    let res = self.eval_function_call(&arg_list, id)?;
                    if let Some(x) = res {
                        self.stack_push(StackValue::Value(x));
                    }
                }
                Instruction::Push { value } => self.stack.push(StackValue::Value(*value)),
                Instruction::PrintLn => {
                    if let StackValue::Value(val) = self.stack.pop().unwrap(){
                        println!("{:?}", val) ;    
                    }
                }
                Instruction::Mul {} => {
                    match self.stack_pop(){
                        Ok(StackValue::Value(x)) => {
                            match self.stack_pop(){
                                Ok(StackValue::Value(y)) => {
                                    let result = x.checked_mul(y)
                                    .ok_or(InterpError::Numeric("overflowed".to_string()))?;
                                    self.stack_push(StackValue::Value(result));   
                                },
                                Err(err) => return Err(err),
                                _ => {}// return Err(InterpError::EvalError("Unexpected stack value type".to_string()))
                            }
                        },
                        Err(err) => return Err(err),
                        _ => {}// return Err(InterpError::EvalError("Unexpected stack value type".to_string()))
                    }
                }
                Instruction::Add {} => {
                    match self.stack_pop(){
                        Ok(StackValue::Value(x)) => {
                            match self.stack_pop(){
                                Ok(StackValue::Value(y)) => {
                                    let result = x.checked_add(y)
                                    .ok_or(InterpError::Numeric("overflowed".to_string()))?;
                                    self.stack_push(StackValue::Value(result));   
                                },
                                Err(err) => return Err(err),
                                _ => {}// return Err(InterpError::EvalError("Unexpected stack value type".to_string()))
                            }
                        },
                        Err(err) => return Err(err),
                        _ => {}// return Err(InterpError::EvalError("Unexpected stack value type".to_string()))
                    }
                }
                Instruction::Assign { id } => {
                    let val = self.stack_pop()?;
                    if let StackValue::Value(x) = val {
                        self.set_var(StackValue::Variable(id.to_string(), x));
                    }
                    
                }
                Instruction::Load { id } => {
                    let var = self.get_var(id.into())?;
                    if let StackValue::Variable(_, val) = var{
                        self.stack_push(StackValue::Value(val));
                    }
                    
                }
            }
        }
        if self.stack.is_empty() {
            return Ok(None);
        }
        
        match self.stack_pop(){
            Ok(StackValue::Value(x)) => {
                return Ok(Some(x));
            },
            Err(err) => return Err(err),
            _ => Ok(None)
        }
    }
}
