#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Command {
    Push(Segment, u16),
    Pop(Segment, u16),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}
