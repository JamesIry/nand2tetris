use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    A(Reference),
    C(HashSet<Register>, Expr, Jump),
    L(String),
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Reference {
    Symbol(String),
    Address(u16),
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Register {
    A,
    D,
    M,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Jump {
    Null,
    Jgt,
    Jeq,
    Jge,
    Jlt,
    Jne,
    Jle,
    Jmp,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Expr {
    Zero,
    One,
    NegOne,
    D,
    A,
    M,
    NotD,
    NotA,
    NotM,
    NegD,
    NegA,
    NegM,
    DAddOne,
    AAddOne,
    MAddOne,
    DSubOne,
    ASubOne,
    MSubOne,
    DAddA,
    DAddM,
    DSubA,
    DSubM,
    ASubD,
    MSubD,
    DAndA,
    DAndM,
    DOrA,
    DOrM,
}
