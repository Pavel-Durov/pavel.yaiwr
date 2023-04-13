#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{BinaryOp, Instruction, StackValue},
        Calc, scope::Scope,
    };

    #[test]
    fn add_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1-2;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first, second, third] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(
                    second,
                    &Instruction::Push {
                        value: StackValue::Integer(2)
                    }
                );
                assert_eq!(third, &Instruction::BinaryOp { op: BinaryOp::Sub });
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic(expected = "?")]
    fn add_overflow_max_i64() {
        let calc = &mut Calc::new();
        let input = format!("{}-{};", i64::MIN, 1);
        
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, &mut Scope::new()).unwrap();
    }

    #[test]
    fn add_no_overflow() {
        let calc = &mut Calc::new();
        let input = format!("{}+{};", i64::MAX - 1, 1);
        let ast = calc.from_str(input.as_str()).unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        calc.eval(&bytecode, &mut Scope::new()).unwrap();
    }
}
