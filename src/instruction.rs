/// 命令セット
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Push(i32),
    Pop,
    Equal,
    LessThan,
    And,
    Or,
    Not,
    JumpIfZero,
    Load,
    Store,
    Input,
    Output,
    Halt,
}
