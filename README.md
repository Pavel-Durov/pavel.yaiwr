# pavel.yaiwr

Yet Another Interpreter Witten In Rust

## CI

[Buildbot](https://ci.soft-dev.org/#/builders/1)

## Usage

### Repl

```shell
$ cargo run 
👉 2+1 
3
```

### Cli args
```shell
$ cargo run 'println(2+2+3);'
6
6
6
```

### File
```shell
$ cargo run ./programs/print.yaiwr
4
```

### Logs

Log levels can be configured via the environment variable: RUST_LOG.
```shell
RUST_LOG=info cargo run -- '2+2'
RUST_LOG=debug cargo run -- '2+2'
RUST_LOG=error  cargo run -- '2+?'
```
env_logger crate [docs](https://docs.rs/env_logger/0.10.0/env_logger/)

## Tests

```shell
# run unit tests
$ cargo test
# run test in a container
$ run_docker_ci_job # optional (--prune)
```

## Langugage Spec(ish)

### Statements

`println` - Prints to the standard output, with a new line

Example:

```
println(1+2);
println(1);
```

### Variables

Variable names:

1. Variable names have to start with "_"
2. Variable names can only include alphanumeric and underscore ("_") characters

Example:
```
let _someVariable = (1+2);
let _someVariable3 = 1;
let _x = 2;
let _y = 1 * _x;
```
Repl: 

# TODOs

[x] Go through the calc example in the quick start guide

[x] Implement a testing framework

[x] Split between "compile an AST to Vec<Opcode> and then have an evaluator which takes Vec<Opcode> and executes the program"

[x] Implement stack-based VM

[x] Implement print statement

[x] Implement variables

[ ] Propogate all errors to top-level where the error is printed

[ ] Compile variable names to integers

[ ] Implement functions

[ ] Add support for custom error handling, i.e InterpError

[ ] Use actions to make an AST on the fly (see e.g. https://github.com/ltratt/pizauth/blob/master/src/config.y)

[ ] Add variable declaration and out-of-parsing-time computation

[ ] Benchmarking

[ ] Revise Rust in general :)

# Terminology

**Parse Tree** - includes all the "useless" syntactic information that humans like/need but doesn't affect compilation

**AST** - strip out that useless syntactic stuff

**Evaluator** - evaluates something (parse tree, AST, or opcodes) directly; a "compiler" converts one thing into another

**Stack-based machines** - Stack for operands and operators, the result is always on top of the stack


# Resources

[Building a Virtual Machine [2/29]: Stack vs. Register VM](https://www.youtube.com/watch?v=7hrLD4z8eUA&ab_channel=DmitrySoshnikov)

[Which Parsing Approach?](https://tratt.net/laurie/blog/2020/which_parsing_approach.html)

[Yacc](https://web.archive.org/web/20220830093827/dinosaur.compilertools.net/yacc/index.html)

[Quickstart](https://softdevteam.github.io/grmtools/master/book/quickstart.html)
Yet Another Interpreter Written In Rust
