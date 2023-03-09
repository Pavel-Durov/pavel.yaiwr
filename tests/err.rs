mod print;

#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn eval_error() {
        let c = Calc::new();
        let parsing_err = c.from_str("invalid input").err().unwrap();
        assert_ne!(parsing_err, "")
    }
}
