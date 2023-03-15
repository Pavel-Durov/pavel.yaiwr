mod print;

#[cfg(test)]
mod tests {
    use std::process::Command;
    use yaiwr::{err::InterpError, Calc};

    #[test]
    fn eval_error() {
        let c = Calc::new();
        let parsing_err = c.from_str("invalid input").err().unwrap();
        assert!(matches!(parsing_err, InterpError::ParseError(..)));
    }

    #[test]
    fn file_notfound_sderr_error() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("-q")
            .arg("imaginary-file.yaiwr")
            .output()
            .expect(format!("command 'cargo run imaginary-file.yaiwr' failed").as_str());
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "Program file: 'imaginary-file.yaiwr' cannot be found!\n"
        );
    }

    #[test]
    fn parse_sderr_error() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("-q")
            .arg("1+2)")
            .output()
            .expect(format!("command 'cargo run ???' failed").as_str());
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "Parse error: Parsing error at line 1 column 4. Repair sequences found:\n   1: Delete )!\n"
        );
    }
}
