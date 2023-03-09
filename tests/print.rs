#[cfg(test)]
mod tests {
    use yaiwr::{instruction::Instruction, Calc};
    #[test]
    fn eval_println_statement_add() {
        let ast = Calc::from_str("println(2+2)").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(None));
    }
    #[test]
    fn eval_println_statement_mul() {
        let ast = Calc::from_str("println(2*2)").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(None));
    }

    #[test]
    fn println_statement_numeric_bytecode() {
        let ast = Calc::from_str("println(1)").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
            [first, second] => {
                assert_eq!(first, &Instruction::Push { value: 1 });
                assert_eq!(second, &Instruction::PrintLn);
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn print_statement_add_bytecode() {
        let ast = Calc::from_str("println (1+1)").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
            [c1, c2, c3, c4] => {
                assert_eq!(c1, &Instruction::Push { value: 1 });
                assert_eq!(c2, &Instruction::Push { value: 1 });
                assert_eq!(c3, &Instruction::Add {});
                assert_eq!(c4, &Instruction::PrintLn {});
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic]
    fn eval_println_statement_add_parsing_error() {
        let ast = Calc::from_str("println 2+2").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(None));
    }
    #[test]
    #[should_panic]
    fn eval_println_statement_mul_parsing_error() {
        let ast = Calc::from_str("println 2*2").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        assert_eq!(Calc::eval(bytecode), Ok(None));
    }

    #[test]
    fn println_cmd() {
        use std::process::Command;
        let output = Command::new("cargo")
            .arg("run")
            .arg("println(2)")
            .output()
            .expect("comand 'cargo run println(2)' failed");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n");
    }
}
