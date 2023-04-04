#[cfg(test)]
mod tests {
    use yaiwr::{
        instruction::{Instruction, StackValue},
        Calc,
    };

    #[test]
    fn integer_literal_bc() {
        let calc = &mut Calc::new();
        let ast = calc.from_str("1").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        match bytecode.as_slice() {
            [first] => {
                assert_eq!(
                    first,
                    &Instruction::Push {
                        value: StackValue::Integer(1)
                    }
                );
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }
}
