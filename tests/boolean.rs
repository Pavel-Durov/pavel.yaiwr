#[cfg(test)]
mod tests {
    use yaiwr::{instruction::{Instruction, StackValue}, Calc, ast::AstNode};

    #[test]
    fn bool_literal_true_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("true").unwrap();
        assert_eq!(ast[0], AstNode::Boolean{ value: true });
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(first, &Instruction::Push { value: StackValue::Boolean(true) });
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
    
    #[test]
    fn bool_literal_false_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("true").unwrap();
        assert_eq!(ast[0], AstNode::Boolean{ value: true });
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(first, &Instruction::Push { value: StackValue::Boolean(true) });
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
