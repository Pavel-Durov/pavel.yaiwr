#[derive(Debug, Clone, PartialEq)]
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
    Boolean {
        value: bool,
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
        block: Vec<AstNode>,
    },
    FunctionCall {
        id: String,
        args: Vec<AstNode>,
    },
    Return {
        block: Box<AstNode>,
    },
    GreaterThan {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    LessThan {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
}
