#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use yaiwr::{
        err::InterpError,
        hash::HashId,
        instruction::{BinaryOp, EvalResult, Instruction, StackValue},
        scope::Scope,
        YIWR,
    };

    pub fn eval_prog<'a>(
        yaiwr: &mut YIWR,
        input: &str,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Option<EvalResult>, InterpError> {
        let ast = yaiwr.from_str(input).unwrap();
        let bytecode = YIWR::ast_to_bytecode(ast);
        return yaiwr.eval(&bytecode, scope);
    }

    #[test]
    fn function_call_err() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        eval_prog(yaiwr, "fun add1 (_p1){ return _p1 + 1; }", scope.clone()).unwrap();
        assert_eq!(
            eval_prog(yaiwr, "add1();", scope),
            Err(InterpError::EvalError(
                "Unexpected number of function arguments. Expected: 1, Got: 0".to_string()
            ))
        );
    }

    #[test]
    fn sss() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        let result = eval_prog(yaiwr, "
          fun f1(f2) {
            return f2() + 1;
          }
          
          fun f2() {
            return 2;
          }
          
          let result = f1(f2);
          
          println(result);
        
        ", scope.clone());
        assert_eq!(
            result,
            Ok(Some(EvalResult::Value(StackValue::Integer(3))))
        );
    }

    #[test]
    fn function_undefined_err() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        assert_eq!(
            eval_prog(yaiwr, "add1();", scope),
            Err(InterpError::UndefinedFunction("add1".to_string()))
        );
    }

    #[test]
    fn function_composition() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        eval_prog(yaiwr, "fun add1 (_p1){ return _p1 + 1; }", scope.clone()).unwrap();
        eval_prog(yaiwr, "fun add2 (_p1){ return _p1 + 2; }", scope.clone()).unwrap();
        assert_eq!(
            eval_prog(yaiwr, "add2(add1(1));", scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(4))
        );
    }

    #[test]
    fn function_multiple_params() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        eval_prog(
            yaiwr,
            "fun add (p1, p2, p3){ return p1 + p2 +p3; }",
            scope.clone(),
        )
        .unwrap();
        assert_eq!(
            eval_prog(yaiwr, "add(1,2,3);", scope).unwrap().unwrap(),
            EvalResult::Value(StackValue::Integer(6))
        );
    }

    #[test]
    fn function_declaration_no_params_bc() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        let prog1 = "fun some (){ return 2*2; }";
        let ast = yaiwr.from_str(prog1).unwrap();
        let func_declare_bc = YIWR::ast_to_bytecode(ast);
        yaiwr.eval(&func_declare_bc, scope.clone()).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        name: "some".to_string(),
                        id: "some".id(),
                        scope: Some(scope.clone()),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::BinaryOp { op: BinaryOp::Mul }
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_bc() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        let prog = "fun add (_p1, _p2){ return _p1 + _p2 + 1; }";
        let ast = yaiwr.from_str(prog).unwrap();
        let func_declare_bc = YIWR::ast_to_bytecode(ast);
        yaiwr.eval(&func_declare_bc, scope.clone()).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        name: "add".to_string(),
                        id: "add".id(),
                        scope: Some(scope.clone()),
                        params: vec!["_p1".to_string(), "_p2".to_string()],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Load {
                                    id: "_p1".to_string()
                                },
                                Instruction::Load {
                                    id: "_p2".to_string()
                                },
                                Instruction::BinaryOp { op: BinaryOp::Add },
                                Instruction::Push {
                                    value: StackValue::Integer(1)
                                },
                                Instruction::BinaryOp { op: BinaryOp::Add },
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_call_bc() {
        let scope = Rc::new(RefCell::new(Scope::new()));
        let yaiwr = &mut YIWR::new();
        let prog_func_declaration = "fun add (_p1, _p2){ return _p1 + _p2; }";
        let ast = yaiwr.from_str(prog_func_declaration).unwrap();
        let func_declaration_bc = YIWR::ast_to_bytecode(ast);
        yaiwr.eval(&func_declaration_bc, scope.clone()).unwrap();
        match func_declaration_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        name: "add".to_string(),
                        id: "add".id(),
                        scope: Some(scope.clone()),
                        params: vec!["_p1".to_string(), "_p2".to_string()],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Load {
                                    id: "_p1".to_string()
                                },
                                Instruction::Load {
                                    id: "_p2".to_string()
                                },
                                Instruction::BinaryOp { op: BinaryOp::Add },
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }

        let prog_func_call = "add(1,2);";
        let ast = yaiwr.from_str(prog_func_call).unwrap();
        let func_call_bc = YIWR::ast_to_bytecode(ast);
        
        yaiwr.eval(&func_call_bc, scope.clone()).unwrap();
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "add".to_string(),
                        args: vec![
                            vec![Instruction::Push {
                                value: StackValue::Integer(1)
                            }],
                            vec![Instruction::Push {
                                value: StackValue::Integer(2)
                            }],
                        ]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_no_params_call_bc() {
        let yaiwr = &mut YIWR::new();
        let prog_func_declaration = "fun two_plus_two (){ return (2+2); }";
        let ast = yaiwr.from_str(prog_func_declaration).unwrap();
        let func_declare_bc = YIWR::ast_to_bytecode(ast);
        let scope = Rc::new(RefCell::new(Scope::new()));
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        name: "two_plus_two".to_string(),
                        id: "two_plus_two".id(),
                        params: vec![],
                        scope: Some(scope.clone()),
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::Push {
                                    value: StackValue::Integer(2)
                                },
                                Instruction::BinaryOp { op: BinaryOp::Add },
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
        let prog_func_call = "two_plus_two();";
        let ast = yaiwr.from_str(prog_func_call).unwrap();
        let func_call_bc = YIWR::ast_to_bytecode(ast);
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "two_plus_two".to_string(),
                        args: vec![]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
