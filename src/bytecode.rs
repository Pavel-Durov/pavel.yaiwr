use crate::{ast::AstNode, instruction::Instruction};

fn function_call(id: String, args: Vec<AstNode>, prog: &mut Vec<Instruction>) {
    let mut args_bytecode = vec![];
    for a in args {
        let bytecode = &mut vec![];
        to_bytecode(a, bytecode);
        args_bytecode.push(bytecode.to_vec());
    }
    prog.push(Instruction::FunctionCall {
        id,
        args: args_bytecode,
    })
}

fn function_ast_params_to_vec(params: Vec<AstNode>) -> Vec<String> {
    let mut bytecode = vec![];
    for p in params {
        if let AstNode::ID { value } = p {
            bytecode.push(value)
        }
    }
    return bytecode;
}

fn function_declaration(
    id: String,
    params: Vec<AstNode>,
    block: AstNode,
    prog: &mut Vec<Instruction>,
) {
    let bytecode = &mut vec![];
    to_bytecode(block, bytecode);
    let parsed_params = function_ast_params_to_vec(params);
    prog.push(Instruction::Function {
        id,
        block: bytecode.to_vec(),
        params: parsed_params,
    });
}

pub fn to_bytecode(ast_node: AstNode, prog: &mut Vec<Instruction>) {
    match ast_node {
        AstNode::Return { block: body } => {
            let bytecode = &mut vec![];
            to_bytecode(*body, bytecode);
            prog.push(Instruction::Return {
                block: bytecode.to_vec(),
            });
        }
        AstNode::FunctionCall { id, args } => function_call(id, args, prog),
        AstNode::Function { id, params, block } => function_declaration(id, params, *block, prog),
        AstNode::Add { lhs, rhs } => {
            to_bytecode(*lhs, prog);
            to_bytecode(*rhs, prog);
            prog.push(Instruction::Add {})
        }
        AstNode::Mul { lhs, rhs } => {
            to_bytecode(*lhs, prog);
            to_bytecode(*rhs, prog);
            prog.push(Instruction::Mul {})
        }
        AstNode::Number { value } => prog.push(Instruction::Push { value: value }),
        AstNode::PrintLn { rhs } => {
            to_bytecode(*rhs, prog);
            prog.push(Instruction::PrintLn {})
        }
        AstNode::Assign { id, rhs } => {
            to_bytecode(*rhs, prog);
            prog.push(Instruction::Assign { id })
        }
        AstNode::ID { value } => prog.push(Instruction::Load { id: value }),
    }
}
