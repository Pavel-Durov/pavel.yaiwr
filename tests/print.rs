#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{BinaryOp, Instruction, StackValue},
        Calc,
    };

    #[test]
    fn println_statement_numeric_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println(1);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first, second] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(second, &Instruction::PrintLn);
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn print_statement_add_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("println (1+1);").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [c1, c2, c3, c4] => {
                assert_eq!(
                    c1,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(
                    c2,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(c3, &Instruction::BinaryOp { op: BinaryOp::Add });
                assert_eq!(c4, &Instruction::PrintLn {});
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
