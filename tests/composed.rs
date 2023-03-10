#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn eval_mul_and_plus_expressions() {
        let bytecode = &mut vec![];
        let mut c = Calc::new();
        c.to_bytecode(c.from_str("2*3+2").unwrap(), bytecode);
        assert_eq!(c.eval(bytecode).unwrap(), Some(8), "expected 2*3+2=8");
        let bytecode = &mut vec![];
        c.to_bytecode(c.from_str("2+3*2").unwrap(), bytecode);
        assert_eq!(c.eval(bytecode).unwrap(), Some(8), "expected 2+3*2=8");
        let bytecode = &mut vec![];
        c.to_bytecode(c.from_str("(2+3)*2").unwrap(), bytecode);
        assert_eq!(c.eval(bytecode).unwrap(), Some(10), "expected (2+3)*2=10");
    }
}
