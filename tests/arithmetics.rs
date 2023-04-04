#[cfg(test)]

mod tests {
    use yaiwr::{instruction::StackValue, scope::Scope, Calc};

    #[test]
    fn eval_mul_and_plus_expressions() {
        let mut c = Calc::new();
        let bytecode = Calc::ast_to_bytecode(c.from_str("2*3+2").unwrap());
        assert_eq!(
            c.eval(&bytecode, &mut Scope::new()).unwrap(),
            Some(StackValue::Integer(8)),
            "expected 2*3+2=8"
        );

        let bytecode = Calc::ast_to_bytecode(c.from_str("2+3*2").unwrap());
        assert_eq!(
            c.eval(&bytecode, &mut Scope::new()).unwrap(),
            Some(StackValue::Integer(8)),
            "expected 2+3*2=8"
        );

        let bytecode = Calc::ast_to_bytecode(c.from_str("(2+3)*2").unwrap());
        assert_eq!(
            c.eval(&bytecode, &mut Scope::new()).unwrap(),
            Some(StackValue::Integer(10)),
            "expected (2+3)*2=8"
        );
    }
}
