use log::debug;
use std::{
    env, fs,
    io::{self, stdout, BufRead, Write},
};
use yaiwr::Calc;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("cli args {:?}", &args[1..]);
    if args.len() > 1 {
        if args[1].ends_with(".yaiwr") {
            run_from_file(&args[1])
        } else {
            eval_line(&args[1]);
        }
    } else {
        repl();
    }
}

fn run_from_file(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .collect();
    for line in lines {
        eval_line(line);
    }
}

fn repl() {
    let stdin = io::stdin();
    loop {
        print!("👉 ");
        stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                if let Some(value) = eval_line(l) {
                    println!("{}", value);
                }
            }
            _ => break,
        }
    }
}

fn eval_line(input: &str) -> Option<u64> {
    debug!("input: {:?}", &input);
    let mut c = Calc::new();
    let ast = c.from_str(input);
    match ast {
        Ok(ast_node) => {
            debug!("AST: {:?}", &ast_node);
            let bytecode = &mut vec![];
            c.to_bytecode(ast_node, bytecode);
            debug!("Bytecode: {:?}", &bytecode);
            match c.eval(bytecode) {
                Ok(result) => {
                    return result;
                }
                Err(msg) => {
                    eprintln!("Evaluation error: {}", msg);
                    None
                }
            }
        }
        Err(msg) => {
            eprintln!("Evaluation error: {}", msg);
            None
        }
    }
}
