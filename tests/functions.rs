#[cfg(test)]
mod tests {
    use std::process::Command;

    use yaiwr::{instruction::Instruction, Calc};

    #[test]
    fn var_test_file_expect_output_1984() {
        let cmd = "'fun _some (){ 2+2 }'";
        let output = Command::new("cargo")
            .arg("run")
            .arg(cmd)
            .output()
            .expect(format!("comand 'cargo run {}' failed", cmd).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "",);
    }
    #[test]
    fn function_declaration_and_call() {
        let calc = &mut Calc::new();
        let prog1 = "fun _two_plus_two (){ return (2+2); }";
        let ast = calc.from_str(prog1).unwrap();
        let bytecode = &mut vec![];
        calc.to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
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
        let prog1 = "_two_plus_two();";
        let ast = calc.from_str(prog1).unwrap();
        let bytecode = &mut vec![];
        calc.to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::FunctionCall { id: "_two_plus_two".to_string(), args: vec![] }
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
