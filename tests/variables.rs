#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn variables_declaration() {
        struct TestCase<'a> {
            prog: &'a str,
            key: &'a str,
            expected_value: u64,
        }
        let mut c = Calc::new();
        let table = vec![
            TestCase {
                prog: "let _a = 2;",
                key: "_a",
                expected_value: 2,
            },
            TestCase {
                prog: "let _b = (1+2*3);",
                key: "_b",
                expected_value: 7,
            },
            TestCase {
                prog: "let _c = 1+2+3+4;",
                key: "_c",
                expected_value: 10,
            },
            TestCase {
                prog: "let _dA1 = 6;",
                key: "_dA1",
                expected_value: 6,
            },
            TestCase {
                prog: "let _ABCDabc123 = 1984;",
                key: "_ABCDabc123",
                expected_value: 1984,
            },
        ];
        for t in table {
            let ast = c.from_str(t.prog).unwrap();
            let bytecode = &mut vec![];
            c.to_bytecode(ast, bytecode);
            c.eval(bytecode).unwrap();
            assert_eq!(c.get_var(t.key.to_string()), &t.expected_value);
        }
    }

    #[test]
    fn variables_expression() {
        use std::process::Command;
        struct TestCase<'a> {
            file_path: &'a str,
            expected_output: &'a str,
        }
        let table = vec![
            TestCase {
                file_path: "./programs/tests/var_expect_output_10.yaiwr",
                expected_output: "10\n",
            },
            TestCase {
                file_path: "./programs/tests/var_expect_output_1984.yaiwr",
                expected_output: "1984\n",
            },
        ];
        for t in table {
            let output = Command::new("cargo")
                .arg("run")
                .arg(t.file_path)
                .output()
                .expect(format!("comand 'cargo run {}' failed", t.file_path).as_str());

            assert_eq!(String::from_utf8_lossy(&output.stdout), t.expected_output);
        }
    }
}
