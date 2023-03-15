#[derive(Debug, Clone)]
pub enum AstNode {
    Add {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Mul {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Number {
        value: u64,
    },
    ID {
        value: String,
    },
    PrintLn {
        rhs: Box<AstNode>,
    },
    Assign {
        id: String,
        rhs: Box<AstNode>,
    },
    Function {
        id: String,
        params: Vec<AstNode>,
        block: Box<AstNode>,
    },
    FunctionCall {
        id: String,
        args: Vec<AstNode>,
    },
    Return {
        block: Box<AstNode>,
    },
}
