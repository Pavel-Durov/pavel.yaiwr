#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add,
    Mul,
    Push { value: u64 },
    PrintLn,
    Assign { id: String },
    Load { id: String },
}
