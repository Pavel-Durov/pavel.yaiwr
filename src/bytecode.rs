use crate::{
    ast::AstNode,
    instruction::{BinaryOp, Instruction, StackValue},
};

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

pub fn block_to_bytecode(block: Vec<AstNode>) -> Vec<Instruction> {
    let bytecodes = &mut vec![];
    for n in block {
        let bytecode = &mut vec![];
        to_bytecode(n, bytecode);
        bytecodes.append(bytecode);
    }
    return bytecodes.to_vec();
}

fn function_declaration(
    id: String,
    params: Vec<AstNode>,
    block: Vec<AstNode>,
    prog: &mut Vec<Instruction>,
) {
    let parsed_params = function_ast_params_to_vec(params);
    prog.push(Instruction::Function {
        id,
        block: block_to_bytecode(block),
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
        AstNode::Function { id, params, block } => function_declaration(id, params, block, prog),
        AstNode::Add { lhs, rhs } => {
            to_bytecode(*lhs, prog);
            to_bytecode(*rhs, prog);
            prog.push(Instruction::BinaryOp { op: BinaryOp::Add })
        }
        AstNode::Mul { lhs, rhs } => {
            to_bytecode(*lhs, prog);
            to_bytecode(*rhs, prog);
            prog.push(Instruction::BinaryOp { op: BinaryOp::Mul })
        }
        AstNode::Number { value } => prog.push(Instruction::Push {
            value: StackValue::Integer(value),
        }),
        AstNode::PrintLn { rhs } => {
            to_bytecode(*rhs, prog);
            prog.push(Instruction::PrintLn {})
        }
        AstNode::Assign { id, rhs } => {
            to_bytecode(*rhs, prog);
            prog.push(Instruction::BinaryOp {
                op: BinaryOp::Assign { id },
            })
        }
        AstNode::ID { value } => prog.push(Instruction::Load { id: value }),
        AstNode::Boolean { value } => prog.push(Instruction::Push {
            value: StackValue::Boolean(value),
        }),
        AstNode::GreaterThan { lhs, rhs } => {
            to_bytecode(*lhs, prog);
            to_bytecode(*rhs, prog);
            prog.push(Instruction::BinaryOp {
                op: BinaryOp::GreaterThan {},
            })
        }
        AstNode::LessThan { lhs, rhs } => {
            to_bytecode(*lhs, prog);
            to_bytecode(*rhs, prog);
            prog.push(Instruction::BinaryOp {
                op: BinaryOp::LessThan {},
            })
        }
    }
}
