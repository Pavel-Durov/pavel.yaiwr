#[cfg(test)]
mod tests {
    use yaiwr::{instruction::Instruction, Calc};

    #[test]
    fn eval_mul_expression() {
        let mut c = Calc::new();
        let ast = c.from_str("2*2").unwrap();
        let bytecode = &mut vec![];
        c.to_bytecode(ast, bytecode);
        assert_eq!(c.eval(bytecode).unwrap(), Some(4));
    }

    #[test]
    fn eval_mul_expressions() {
        let mut c = Calc::new();
        let ast = c.from_str("2*2*2").unwrap();
        let bytecode = &mut vec![];
        c.to_bytecode(ast, bytecode);
        assert_eq!(c.eval(bytecode).unwrap(), Some(8));
    }

    #[test]
    fn mul_bytecode() {
        let c = Calc::new();
        let ast = c.from_str("1*2").unwrap();
        let bytecode = &mut vec![];
        c.to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
            [first, second, third] => {
                assert_eq!(first, &Instruction::Push { value: 1 });
                assert_eq!(second, &Instruction::Push { value: 2 });
                assert_eq!(third, &Instruction::Mul);
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic(expected = "overflowed")]
    fn mul_overflow() {
        let mut c = Calc::new();
        let input = format!("{}*{}", u64::MAX, 2);
        let ast = c.from_str(input.as_str()).unwrap();
        let bytecode = &mut vec![];
        c.to_bytecode(ast, bytecode);
        c.eval(bytecode).unwrap();
    }

    #[test]
    fn mul_no_overflow() {
        let mut c = Calc::new();
        let input = format!("{}*{}", u64::MAX, 1);
        let ast = c.from_str(input.as_str()).unwrap();
        let bytecode = &mut vec![];
        c.to_bytecode(ast, bytecode);
        c.eval(bytecode).unwrap();
    }
}
