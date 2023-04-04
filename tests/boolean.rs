#[cfg(test)]
mod tests {
    use yaiwr::{
        ast::AstNode,
        instruction::{BinaryOp, Instruction, StackValue},
        Calc,
    };

    #[test]
    fn bool_literal_true_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("true").unwrap();
        assert_eq!(ast[0], AstNode::Boolean { value: true });
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Boolean(true)
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn bool_literal_false_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("true").unwrap();
        assert_eq!(ast[0], AstNode::Boolean { value: true });
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Boolean(true)
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn bool_greater_than_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1 > 2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [bc1, bc2, bc3] => {
                assert_eq!(
                    bc1,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(
                    bc2,
                    &Instruction::Push {
                        value: StackValue::Integer(2)
                    }
                );
                assert_eq!(
                    bc3,
                    &Instruction::BinaryOp {
                        op: BinaryOp::GreaterThan
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
    #[test]
    fn bool_less_than_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1 < 2").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [bc1, bc2, bc3] => {
                assert_eq!(
                    bc1,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(
                    bc2,
                    &Instruction::Push {
                        value: StackValue::Integer(2)
                    }
                );
                assert_eq!(
                    bc3,
                    &Instruction::BinaryOp {
                        op: BinaryOp::LessThan
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    fn bool_less_than_expression_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("(1+2) < 4").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [bc1, bc2, bc3, bc4, bc5] => {
                assert_eq!(
                    bc1,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
                assert_eq!(
                    bc2,
                    &Instruction::Push {
                        value: StackValue::Integer(2)
                    }
                );
                assert_eq!(bc3, &Instruction::BinaryOp { op: BinaryOp::Add });
                assert_eq!(
                    bc4,
                    &Instruction::Push {
                        value: StackValue::Integer(4)
                    }
                );
                assert_eq!(
                    bc5,
                    &Instruction::BinaryOp {
                        op: BinaryOp::LessThan
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
