mod print;

#[cfg(test)]
mod tests {
    use std::process::Command;

    use yaiwr::{Calc, scope::Scope};

    #[test]
    fn comment_no_evaluation_bc() {
        let calc = &mut Calc::new();
        let scope = &mut Scope::new();
        let ast = calc.from_str("// let _a = 5;\n").unwrap();
        let bytecode = Calc::ast_to_bytecode(ast);
        assert_eq!(calc.eval(&bytecode, scope).unwrap(), None);
        assert_eq!(bytecode.len(), 0);
    }

    #[test]
    fn comment_expected_output_4() {
        let file_path = "./programs/tests/comments_expect_output_4.yaiwr";
        let output = Command::new("cargo")
            .arg("run")
            .arg(file_path)
            .output()
            .expect(format!("comand 'cargo run {}' failed", file_path).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "4",);
    }
}