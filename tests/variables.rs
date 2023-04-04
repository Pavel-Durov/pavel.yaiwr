#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{BinaryOp, Instruction, StackValue},
        Calc,
    };

    #[test]
    fn var_assign_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("let _ABCDabc123 = 1984;").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first, second] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Integer(1984)
                    }
                );
                assert_eq!(
                    second,
                    &Instruction::BinaryOp {
                        op: BinaryOp::Assign {
                            id: "_ABCDabc123".to_string()
                        }
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
