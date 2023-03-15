#[cfg(test)]
mod tests {
    use std::{process::Command};
    use yaiwr::{Calc, Scope};

    fn eval_prog(input: &str, scope: &mut Scope)  {
        let mut c = Calc::new();
        let ast = c.from_str(input).unwrap();
        let bytecode = &mut vec![];
        c.to_bytecode(ast, bytecode);
        c.eval_with_scope(bytecode, scope).unwrap();        
    }

    #[test]
    fn var_single_numeric() {
        let scope = &mut Scope::new();
        eval_prog("let _a = 2;", scope);
        assert_eq!(scope.get_var("_a".to_string()).unwrap(), &2);
    }
    #[test]
    fn var_expression() {
        let scope = &mut Scope::new();
        eval_prog("let _b = (1+2*3);", scope);
        assert_eq!(scope.get_var("_b".to_string()).unwrap(), &7);
    }

    #[test]
    fn var_multiple_lower_upper_numeric() {
        let scope = &mut Scope::new();
        eval_prog("let _ABCDabc123 = 1984;", scope);
        assert_eq!(scope.get_var("_ABCDabc123".to_string()).unwrap(), &1984);
    }

    #[test]
    fn var_single_lower_upper_numeric() {
        let scope = &mut Scope::new();
        eval_prog("let _aB1 = 1984;", scope);
        assert_eq!(scope.get_var("_aB1".to_string()).unwrap(), &1984);
    }

    #[test]
    fn var_test_file_expect_output_10() {
        let file_path = "./programs/tests/var_expect_output_10.yaiwr";
        let output = Command::new("cargo")
            .arg("run")
            .arg(file_path)
            .output()
            .expect(format!("comand 'cargo run {}' failed", file_path).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "10\n",);
    }

    #[test]
    fn var_test_file_expect_output_1984() {
        let file_path = "./programs/tests/var_expect_output_1984.yaiwr";
        let output = Command::new("cargo")
            .arg("run")
            .arg(file_path)
            .output()
            .expect(format!("comand 'cargo run {}' failed", file_path).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "1984\n",);
    }
}
