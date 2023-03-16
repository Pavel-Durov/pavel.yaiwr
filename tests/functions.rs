#[cfg(test)]
mod tests {
    use yaiwr::{err::InterpError, instruction::Instruction, Calc};

    pub fn eval_prog(calc: &mut Calc, input: &str) -> Result<Option<u64>, InterpError> {
        let ast = calc.from_str(input).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        return calc.eval(&bytecode);
    }

    #[test]
    fn function_call_err() {
        let calc = &mut Calc::new();
        eval_prog(calc, "fun _add1 (_p1){ return _p1 + 1; }").unwrap();
        assert_eq!(eval_prog(calc, "_add1()"), Err(InterpError::EvalError("Unexpected number of function arguments. Expected: 1, Got: 0".to_string())));
    }

    #[test]
    fn function_undefined_err() {
        let calc = &mut Calc::new();
        assert_eq!(eval_prog(calc, "_add1()"), Err(InterpError::UndefinedFunction("_add1".to_string())));
    }

    #[test]
    fn function_composition() {
        let calc = &mut Calc::new();
        eval_prog(calc, "fun _add1 (_p1){ return _p1 + 1; }").unwrap();
        eval_prog(calc, "fun _add2 (_p1){ return _p1 + 2; }").unwrap();
        assert_eq!(eval_prog(calc, "_add2(_add1(1))").unwrap().unwrap(), 4);
    }

    #[test]
    fn function_multiple_params() {
        let calc = &mut Calc::new();
        eval_prog(calc, "fun _add (_p1, _p2, _p3){ return _p1 + _p2 +_p3; }").unwrap();
        assert_eq!(eval_prog(calc, "_add(1,2,3)").unwrap().unwrap(), 6);
    }

    #[test]
    fn function_params_as_variables() {
        let calc = &mut Calc::new();
        eval_prog(calc, "let _x = 2;").unwrap();
        eval_prog(calc, "let _y = 3;").unwrap();
        eval_prog(calc, "fun _add (_arg1, _arg2){ return _arg1 + _arg2; }").unwrap();
        assert_eq!(eval_prog(calc, "_add(_x, _y)").unwrap().unwrap(), 5);
    }

    #[test]
    fn function_call_from_function_call() {
        let calc = &mut Calc::new();
        eval_prog(calc, "let _x = 2;").unwrap();
        eval_prog(calc, "let _y = 3;").unwrap();
        eval_prog(
            calc,
            "fun _add (_arg1, _arg2){ return _id(_arg1) + _id(_arg2); }",
        )
        .unwrap();
        eval_prog(calc, "fun _id (_arg1){ return _arg1; }").unwrap();
        assert_eq!(eval_prog(calc, "_add(_x, _y)").unwrap().unwrap(), 5);
    }

    #[test]
    fn function_declaration_no_params_bytecode() {
        let calc = &mut Calc::new();
        let prog1 = "fun _some (){ return 2*2; }";
        let ast = calc.from_str(prog1).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "_some".to_string(),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push { value: 2 },
                                Instruction::Push { value: 2 },
                                Instruction::Mul
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_bytecode() {
        let calc = &mut Calc::new();
        let prog = "fun _add (_p1, _p2){ return _p1 + _p2 + 1; }";
        let ast = calc.from_str(prog).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "_add".to_string(),
                        params: vec!["_p1".to_string(), "_p2".to_string()],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Load {
                                    id: "_p1".to_string()
                                },
                                Instruction::Load {
                                    id: "_p2".to_string()
                                },
                                Instruction::Add,
                                Instruction::Push { value: 1 },
                                Instruction::Add
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_with_params_call_bytecode() {
        let calc = &mut Calc::new();
        let prog_func_declaration = "fun _add (_p1, _p2){ return _p1 + _p2; }";
        let ast = calc.from_str(prog_func_declaration).unwrap();
        let func_declaration_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declaration_bc).unwrap();
        match func_declaration_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "_add".to_string(),
                        params: vec!["_p1".to_string(), "_p2".to_string()],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Load {
                                    id: "_p1".to_string()
                                },
                                Instruction::Load {
                                    id: "_p2".to_string()
                                },
                                Instruction::Add
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }

        let prog_func_call = "_add(1,2)";
        let ast = calc.from_str(prog_func_call).unwrap();
        let func_call_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_call_bc).unwrap();
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "_add".to_string(),
                        args: vec![
                            vec![Instruction::Push { value: 1 }],
                            vec![Instruction::Push { value: 2 }],
                        ]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn function_declaration_no_params_call_bytecode() {
        let calc = &mut Calc::new();
        let prog_func_declaration = "fun _two_plus_two (){ return (2+2); }";
        let ast = calc.from_str(prog_func_declaration).unwrap();
        let func_declare_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_declare_bc).unwrap();
        match func_declare_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Function {
                        id: "_two_plus_two".to_string(),
                        params: vec![],
                        block: vec![Instruction::Return {
                            block: vec![
                                Instruction::Push { value: 2 },
                                Instruction::Push { value: 2 },
                                Instruction::Add
                            ]
                        }]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
        let prog_func_call = "_two_plus_two()";
        let ast = calc.from_str(prog_func_call).unwrap();
        let func_call_bc = Calc::ast_to_bytecode(ast);
        calc.eval(&func_call_bc).unwrap();
        match func_call_bc.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall {
                        id: "_two_plus_two".to_string(),
                        args: vec![]
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn test_functions_args() {
        use std::process::Command;
        let output = Command::new("cargo")
            .arg("run")
            .arg("programs/tests/functions_expect_output_15.yaiwr")
            .output()
            .expect("command 'cargo run programs/tests/functions_expect_output_15.yaiwr' failed");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "15\n");
    }
}
