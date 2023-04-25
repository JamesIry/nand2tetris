#[derive(Debug, Eq, PartialEq)]
pub struct Class {
    pub name: String,
    pub vars: Vec<ClassVarDecl>,
    pub subroutines: Vec<Subroutine>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ClassVarDecl {
    pub decorator: ClassVarDecorator,
    pub type_name: Type,
    pub declarations: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClassVarDecorator {
    Static,
    Field,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    Class(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Subroutine {
    pub decorator: SubroutineDecorator,
    pub type_name: Option<Type>,
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub vars: Vec<SubroutineVarDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SubroutineVarDecl {
    pub type_name: Type,
    pub declarations: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParamDecl {
    pub type_name: Type,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SubroutineDecorator {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    //let name[index]=expr;
    Let {
        name: String,
        index: Option<Expr>,
        expr: Expr,
    },
    //if (expr) {statemens} else {else_statements}
    If {
        condition: Expr,
        statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    },
    // while(condition) {statements}
    While {
        condition: Expr,
        statements: Vec<Statement>,
    },
    // do expr
    Do {
        expr: Expr,
    },
    // return expr
    Return {
        expr: Option<Expr>,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Expr {
    pub term: Box<Term>,
    pub ops: Vec<(Op, Term)>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Term {
    IntegerLit(u16),
    StringLit(String),
    True,
    False,
    Null,
    This,
    Var {
        name: String,
        index: Option<Expr>,
    },
    Bracketed(Expr),
    Unary(UnaryOp, Box<Term>),
    SubroutineCall {
        qualifier: Option<String>,
        name: String,
        exprs: Vec<Expr>,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}
