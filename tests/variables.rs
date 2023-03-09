#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    struct TestCase<'a> {
        prog: &'a str,
        key: &'a str,
        expected_value: u64,
    }

    #[test]
    fn declare_variables() {
        let mut c = Calc::new();
        let mut table = vec![];
        table.push(TestCase {
            prog: "let _a = 2;",
            key: "_a",
            expected_value: 2,
        });
        table.push(TestCase {
            prog: "let _b = (1+2*3);",
            key: "_b",
            expected_value: 7,
        });
        table.push(TestCase {
            prog: "let _c = 1+2+3+4;",
            key: "_c",
            expected_value: 10,
        });
        for t in table {
            let ast = c.from_str(t.prog).unwrap();
            let bytecode = &mut vec![];
            c.to_bytecode(ast, bytecode);
            c.eval(bytecode).unwrap();
            assert_eq!(c.get_var(t.key.to_string()), Some(&t.expected_value));
        }
    }
}
