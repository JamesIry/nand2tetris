use crate::{
    ast::*,
    symbol_table::{RefType, SubroutineVarDecorator, SymbolTable},
};
use anyhow::Result;
use std::io::{BufWriter, Write};
use thiserror::Error;

pub struct Emitter<'a, T>
where
    T: Write,
{
    writer: BufWriter<T>,
    symbol_table: &'a SymbolTable,
    if_label_number: usize,
    while_label_number: usize,
}

impl<'a, T> Emitter<'a, T>
where
    T: Write,
{
    pub fn new(writer: BufWriter<T>, symbol_table: &'a SymbolTable) -> Self {
        Self {
            writer,
            symbol_table,
            if_label_number: 0,
            while_label_number: 0,
        }
    }

    pub fn emit_class(&mut self, class: &Class) -> Result<()> {
        for subroutine in &class.subroutines {
            self.emit_subroutine(&class.name, subroutine)?;
        }

        Ok(())
    }

    pub fn emit_subroutine(&mut self, class: &str, subroutine: &Subroutine) -> Result<()> {
        self.if_label_number = 0;
        self.while_label_number = 0;
        let (_, _, locals) = self
            .symbol_table
            .lookup_subroutine(class, &subroutine.name)?;
        writeln!(
            self.writer,
            "function {}.{} {}",
            class, subroutine.name, locals
        )?;

        match subroutine.decorator {
            SubroutineDecorator::Constructor => {
                let size = self.symbol_table.lookup_class(class)?;
                writeln!(self.writer, "push constant {}", size)?;
                writeln!(self.writer, "call Memory.alloc 1")?;
                writeln!(self.writer, "pop pointer 0")?;
            }
            SubroutineDecorator::Method => {
                writeln!(self.writer, "push argument 0")?;
                writeln!(self.writer, "pop pointer 0")?;
            }
            SubroutineDecorator::Function => (),
        }

        self.emit_statements(class, &subroutine.name, &subroutine.statements)?;

        Ok(())
    }

    fn emit_statements(
        &mut self,
        class: &str,
        subroutine: &str,
        statements: &[Statement],
    ) -> Result<(), anyhow::Error> {
        for statement in statements {
            self.emit_statement(class, subroutine, statement)?;
        }

        Ok(())
    }

    pub fn emit_statement(
        &mut self,
        class: &str,
        subroutine: &str,
        statement: &Statement,
    ) -> Result<()> {
        match statement {
            Statement::Let { name, index, expr } => {
                self.emit_let_statement(class, subroutine, name, index, expr)
            }
            Statement::If {
                condition,
                statements,
                else_statements,
            } => self.emit_if_statement(class, subroutine, condition, statements, else_statements),
            Statement::While {
                condition,
                statements,
            } => self.emit_while_statement(class, subroutine, condition, statements),
            Statement::Do { expr } => self.emit_do_statement(class, subroutine, expr),
            Statement::Return { expr } => self.emit_return_statement(class, subroutine, expr),
        }
    }

    pub fn emit_if_statement(
        &mut self,
        class: &str,
        subroutine: &str,
        condition: &Expr,
        statements: &[Statement],
        else_statements: &Option<Vec<Statement>>,
    ) -> Result<()> {
        let true_label = format!("IF_TRUE{}", self.if_label_number);
        let false_label = format!("IF_FALSE{}", self.if_label_number);
        let end_label = format!("IF_END{}", self.if_label_number);
        self.if_label_number += 1;

        self.emit_expr(class, subroutine, condition)?;

        writeln!(self.writer, "if-goto {}", true_label)?;
        writeln!(self.writer, "goto {}", false_label)?;
        writeln!(self.writer, "label {}", true_label)?;
        self.emit_statements(class, subroutine, statements)?;
        if else_statements.is_some() {
            writeln!(self.writer, "goto {}", end_label)?;
        }
        writeln!(self.writer, "label {}", false_label)?;
        if let Some(else_statements) = else_statements {
            self.emit_statements(class, subroutine, else_statements)?;
            writeln!(self.writer, "label {}", end_label)?;
        }

        Ok(())
    }

    pub fn emit_let_statement(
        &mut self,
        class: &str,
        subroutine: &str,
        name: &str,
        index: &Option<Expr>,
        expr: &Expr,
    ) -> Result<()> {
        self.emit_expr(class, subroutine, expr)?;

        let (reftype, _, number) = self.symbol_table.lookup_var(class, subroutine, name)?;
        if let Some(index) = index {
            self.emit_expr(class, subroutine, index)?;
            match reftype {
                RefType::ClassRefType(ClassVarDecorator::Static) => {
                    writeln!(self.writer, "push static {}", number)?
                }
                RefType::ClassRefType(ClassVarDecorator::Field) => {
                    writeln!(self.writer, "push this {}", number)?
                }
                RefType::SubroutineRefType(SubroutineVarDecorator::Arg) => {
                    writeln!(self.writer, "push argument {}", number)?
                }
                RefType::SubroutineRefType(SubroutineVarDecorator::Local) => {
                    writeln!(self.writer, "push local {}", number)?
                }
            }
            writeln!(self.writer, "add")?;
            writeln!(self.writer, "pop pointer 1")?;
            writeln!(self.writer, "pop that 0")?;
        } else {
            match reftype {
                RefType::ClassRefType(ClassVarDecorator::Static) => {
                    writeln!(self.writer, "pop static {}", number)?
                }
                RefType::ClassRefType(ClassVarDecorator::Field) => {
                    writeln!(self.writer, "pop this {}", number)?
                }
                RefType::SubroutineRefType(SubroutineVarDecorator::Arg) => {
                    writeln!(self.writer, "pop argument {}", number)?
                }
                RefType::SubroutineRefType(SubroutineVarDecorator::Local) => {
                    writeln!(self.writer, "pop local {}", number)?
                }
            }
        }

        Ok(())
    }

    pub fn emit_while_statement(
        &mut self,
        class: &str,
        subroutine: &str,
        condition: &Expr,
        statements: &[Statement],
    ) -> Result<()> {
        let while_label = format!("WHILE_EXP{}", self.while_label_number);
        let done_label = format!("WHILE_END{}", self.while_label_number);
        self.while_label_number += 1;

        writeln!(self.writer, "label {}", while_label)?;
        self.emit_expr(class, subroutine, condition)?;
        writeln!(self.writer, "not")?;
        writeln!(self.writer, "if-goto {}", done_label)?;
        self.emit_statements(class, subroutine, statements)?;
        writeln!(self.writer, "goto {}", while_label)?;
        writeln!(self.writer, "label {}", done_label)?;

        Ok(())
    }

    pub fn emit_do_statement(&mut self, class: &str, subroutine: &str, expr: &Expr) -> Result<()> {
        self.emit_expr(class, subroutine, expr)?;

        writeln!(self.writer, "pop temp 0")?;
        Ok(())
    }

    pub fn emit_return_statement(
        &mut self,
        class: &str,
        subroutine: &str,
        expr: &Option<Expr>,
    ) -> Result<()> {
        if let Some(expr) = expr {
            self.emit_expr(class, subroutine, expr)?;
        } else {
            writeln!(self.writer, "push constant 0")?;
        }
        writeln!(self.writer, "return")?;
        Ok(())
    }

    pub fn emit_expr(&mut self, class: &str, subroutine: &str, expr: &Expr) -> Result<()> {
        self.emit_term(class, subroutine, &expr.term)?;

        for (op, term) in &expr.ops {
            self.emit_term(class, subroutine, term)?;
            let instr = match op {
                Op::Add => "add",
                Op::Sub => "sub",
                Op::Mult => "call Math.multiply 2",
                Op::Div => "call Math.divide 2",
                Op::And => "and",
                Op::Or => "or",
                Op::Lt => "lt",
                Op::Gt => "gt",
                Op::Eq => "eq",
            };
            writeln!(self.writer, "{}", instr)?;
        }

        Ok(())
    }

    pub fn emit_term(&mut self, class: &str, subroutine: &str, term: &Term) -> Result<()> {
        match term {
            Term::IntegerLit(n) => writeln!(self.writer, "push constant {}", n)?,
            Term::StringLit(s) => self.emit_string_lit(s)?,
            Term::True => {
                writeln!(self.writer, "push constant 0")?;
                writeln!(self.writer, "not")?;
            }
            Term::False => writeln!(self.writer, "push constant 0")?,
            Term::Null => writeln!(self.writer, "push constant 0")?,
            Term::This => writeln!(self.writer, "push pointer 0")?,
            Term::Var { name, index } => self.emit_var_ref(class, subroutine, name, index)?,
            Term::Bracketed(expr) => self.emit_expr(class, subroutine, expr)?,
            Term::Unary(op, term) => {
                self.emit_term(class, subroutine, term.as_ref())?;
                let instr = match op {
                    UnaryOp::Neg => "neg",
                    UnaryOp::Not => "not",
                };
                writeln!(self.writer, "{}", instr)?;
            }
            Term::SubroutineCall {
                qualifier,
                name,
                exprs,
            } => self.emit_subroutine_call(
                class,
                subroutine,
                qualifier.as_ref().map(|s| s.as_str()),
                name,
                exprs,
            )?,
        };
        Ok(())
    }

    pub fn emit_string_lit(&mut self, string: &str) -> Result<()> {
        writeln!(self.writer, "push constant {}", string.len())?;
        writeln!(self.writer, "call String.new 1")?;
        for character in string.chars() {
            writeln!(self.writer, "push constant {}", character as u16)?;
            writeln!(self.writer, "call String.appendChar 2")?;
        }
        Ok(())
    }

    pub fn emit_var_ref(
        &mut self,
        class: &str,
        subroutine: &str,
        name: &str,
        index: &Option<Expr>,
    ) -> Result<()> {
        let (reftype, _, number) = self.symbol_table.lookup_var(class, subroutine, name)?;
        match reftype {
            RefType::ClassRefType(ClassVarDecorator::Static) => {
                writeln!(self.writer, "push static {}", number)?
            }
            RefType::ClassRefType(ClassVarDecorator::Field) => {
                writeln!(self.writer, "push this {}", number)?
            }
            RefType::SubroutineRefType(SubroutineVarDecorator::Arg) => {
                writeln!(self.writer, "push argument {}", number)?
            }
            RefType::SubroutineRefType(SubroutineVarDecorator::Local) => {
                writeln!(self.writer, "push local {}", number)?
            }
        }

        if let Some(index) = index {
            self.emit_expr(class, subroutine, index)?;
            writeln!(self.writer, "add")?;
            writeln!(self.writer, "pop pointer 1")?;
            writeln!(self.writer, "push that 0")?;
        }

        Ok(())
    }

    fn emit_subroutine_call(
        &mut self,
        class: &str,
        subroutine: &str,
        qualifier: Option<&str>,
        name: &str,
        exprs: &[Expr],
    ) -> Result<()> {
        let expr_count = exprs.len();
        let (target_class, args) = match qualifier {
            None => {
                self.emit_term(class, subroutine, &Term::This)?;
                (class.to_string(), expr_count + 1)
            }
            // TODO, using the case of the first character is
            // according to spec, but cheesey. Would be cleaner
            // to look up in the symbol table. But to do that
            // I'd need to add entries into the symbol table for
            // all the intrinsics that will be built in a later
            // chapter. I'm going the easy/cheesey route
            Some(target) if Self::starts_with_upper(target) => (target.to_string(), expr_count),
            Some(target) => {
                self.emit_term(
                    class,
                    subroutine,
                    &Term::Var {
                        name: target.to_string(),
                        index: None,
                    },
                )?;
                let (_, type_name, _) = self.symbol_table.lookup_var(class, subroutine, target)?;
                match type_name {
                    Type::Class(class) => Ok((class, expr_count + 1)),
                    _ => Err(CodeGenError::InvalidMethodTarget),
                }?
            }
        };

        for expr in exprs {
            self.emit_expr(class, subroutine, expr)?;
        }

        writeln!(self.writer, "call {}.{} {}", target_class, name, args)?;
        Ok(())
    }

    fn starts_with_upper(target: &str) -> bool {
        target.starts_with(|c: char| c.is_ascii_uppercase())
    }
}

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("Attempt to call a method on a primitive value")]
    InvalidMethodTarget,
}
