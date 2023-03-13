use log::debug;
use std::{
    env, fs,
    io::{self, stdout, BufRead, Write},
};
use yaiwr::{err::InterpError, Calc};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("cli args {:?}", &args[1..]);
    let calc = &mut Calc::new();
    if args.len() > 1 {
        if args[1].ends_with(".yaiwr") {
            run_from_file(&args[1], calc);
        } else {
            print_result(eval_line(&args[1], calc))
        }
    } else {
        repl(calc);
    }
}

fn print_result(result: Result<Option<u64>, InterpError>) {
    match result {
        Ok(Some(value)) => {
            println!("{}", value);
        }
        Ok(None) => {}
        Err(e) => eprintln!("{}", e),
    }
}

pub fn run_from_file(file_name: &str, calc: &mut Calc) {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            let lines: Vec<&str> = content
                .split("\n")
                .filter(|line| !line.trim().is_empty())
                .collect();
            for line in lines {
                print_result(eval_line(line, calc));
            }
        }
        Err(_) => print_result(Err(InterpError::ProgramFileNotFound(file_name.to_string()))),
    }
}

fn repl(calc: &mut Calc) {
    let stdin = io::stdin();
    loop {
        print!("👉 ");
        stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                print_result(eval_line(l, calc));
            }
            _ => break,
        }
    }
}

fn eval_line(input: &str, calc: &mut Calc) -> Result<Option<u64>, InterpError> {
    debug!("input: {:?}", &input);
    let ast = calc.from_str(input);
    match ast {
        Ok(ast_node) => {
            debug!("AST: {:?}", &ast_node);
            let bytecode = &mut vec![];
            calc.to_bytecode(ast_node, bytecode);
            debug!("Bytecode: {:?}", &bytecode);
            match calc.eval(bytecode) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}
