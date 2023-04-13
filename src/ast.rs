#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Add {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Sub {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Mul {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Number {
        value: i64,
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
    Equal {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    NotEqual {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    LessThan {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Conditional {
        condition: Box<AstNode>,
        block: Vec<AstNode>,
        alternative: Option<Vec<AstNode>>,
    },
    LogicalAnd {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    LogicalOr {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Empty,
}
