use log::{debug, error};
use std::{
    env,
    io::{self, stdout, BufRead, Write},
};
use yaiwr::Calc;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        eval_print(&args[1]);
    } else {
        let stdin = io::stdin();
        loop {
            print!("👉 ");
            stdout().flush().ok();
            match stdin.lock().lines().next() {
                Some(Ok(ref l)) => {
                    if l.trim().is_empty() {
                        continue;
                    }
                    eval_print(l);
                }
                _ => break,
            }
        }
    }
}

fn eval_print(input: &str) {
    debug!("input: {:?}", &input);
    let ast = Calc::from_str(input);
    debug!("AST: {:?}", &ast);
    match ast {
        Ok(ast_node) => {
            let bytecode = &mut vec![];
            Calc::to_bytecode(ast_node, bytecode);
            match Calc::eval(bytecode) {
                Ok(i) => println!("Result: {}", i),
                Err(msg) => {
                    error!("Evaluation error: {}", msg);
                    eprintln!("Evaluation error: {}", msg);
                }
            }
        }
        Err(msg) => {
            error!("Evaluation error: {}", msg);
            eprintln!("Evaluation error: {}", msg);
        }
    }
}
