#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Add,
    Mul,
    Push {
        value: u64,
    },
    PrintLn,
    Assign {
        id: String,
    },
    Load {
        id: String,
    },
    Return {
        block: Vec<Instruction>,
    },
    Function {
        id: String,
        params: Vec<String>,
        block: Vec<Instruction>,
    },
    FunctionCall {
        id: String,
        args: Vec<Vec<Instruction>>,
    },
}
