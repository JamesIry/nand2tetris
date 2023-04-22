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
    Goto(String),
    IfGoto(String),
    Label(String),
    Function(String, u16),
    Call(String, u16),
    Return,
    Comment(String),
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
