use crate::{
    ast::*,
    symbol_table::{ClassSymbolTable, SubroutineSymbolTable, SubroutineVarDecorator, SymbolTable},
};
use std::io::{BufWriter, Result, Write};

pub struct Xml<T>
where
    T: Write,
{
    writer: BufWriter<T>,
}

impl<T> Xml<T>
where
    T: Write,
{
    pub fn new(writer: BufWriter<T>) -> Self {
        Self { writer }
    }

    pub fn start(&mut self, indent: usize, element: &str) -> Result<()> {
        self.indent(indent)?;
        self.opening(element)?;
        writeln!(self.writer)?;
        Ok(())
    }

    pub fn end(&mut self, indent: usize, element: &str) -> Result<()> {
        self.indent(indent)?;
        self.closing(element)?;
        writeln!(self.writer)?;
        Ok(())
    }

    fn opening(&mut self, element: &str) -> Result<()> {
        write!(self.writer, "<")?;
        write!(self.writer, "{}", element)?;
        write!(self.writer, ">")?;
        Ok(())
    }

    fn closing(&mut self, element: &str) -> Result<()> {
        write!(self.writer, "</")?;
        write!(self.writer, "{}", element)?;
        write!(self.writer, ">")?;
        Ok(())
    }

    pub fn leaf(&mut self, indent: usize, element: &str, string: &str) -> Result<()> {
        self.indent(indent)?;
        let string = string.replace('&', "&amp;");
        let string = string.replace('\'', "&quot;");
        let string = string.replace('<', "&lt;");
        let string = string.replace('>', "&gt;");

        self.opening(element)?;

        write!(self.writer, " {} ", string)?;

        self.closing(element)?;
        writeln!(self.writer)?;
        Ok(())
    }

    fn indent(&mut self, indent: usize) -> Result<()> {
        for _ in 0..indent {
            write!(self.writer, "  ")?;
        }
        Ok(())
    }

    pub fn write_ast(&mut self, class: Class) -> Result<()> {
        let indent = 0;
        self.start(indent, "class")?;
        self.keyword(indent + 1, "class")?;
        self.identifier(indent + 1, &class.name)?;
        self.symbol(indent + 1, "{")?;

        for class_var_dec in class.vars {
            self.class_var_dec(indent + 1, class_var_dec)?;
        }
        for subroutine in class.subroutines {
            self.subroutine(indent + 1, subroutine)?;
        }

        self.symbol(indent + 1, "}")?;
        self.end(indent, "class")?;
        Ok(())
    }

    pub fn write_symbol_table(&mut self, symbol_table: &SymbolTable) -> Result<()> {
        self.start(0, "symbol_table")?;
        for (class, table) in symbol_table.classes.iter() {
            self.class_symbol(1, class, table)?;
        }

        self.end(0, "symbol_table")?;
        Ok(())
    }

    fn class_symbol(&mut self, indent: usize, name: &str, table: &ClassSymbolTable) -> Result<()> {
        self.start(indent, "class")?;

        self.leaf(indent + 1, "name", name)?;

        for (name, (decorator, _, number)) in &table.class_vars {
            self.class_var_symbol(indent + 1, name, decorator, *number)?;
        }

        for (subroutine, table) in &table.subroutines {
            self.subroutine_symbol(indent + 1, subroutine, table)?;
        }

        self.end(indent, "class")?;
        Ok(())
    }

    fn class_var_symbol(
        &mut self,
        indent: usize,
        name: &str,
        decorator: &ClassVarDecorator,
        number: usize,
    ) -> Result<()> {
        let var_type = match decorator {
            ClassVarDecorator::Static => "static",
            ClassVarDecorator::Field => "field",
        };
        self.var_symbol(indent, var_type, name, number)
    }

    fn subroutine_symbol(
        &mut self,
        indent: usize,
        name: &str,
        table: &SubroutineSymbolTable,
    ) -> Result<()> {
        let decorator = table.decorator;
        let sub_type = match decorator {
            SubroutineDecorator::Constructor => "constructor",
            SubroutineDecorator::Function => "function",
            SubroutineDecorator::Method => "method",
        };
        self.start(indent, sub_type)?;
        self.leaf(indent + 1, "name", name)?;

        for (var, (decorator, _, number)) in &table.vars {
            self.subroutine_var_symbol(indent + 1, var, decorator, *number)?;
        }

        self.end(indent, sub_type)?;
        Ok(())
    }

    fn subroutine_var_symbol(
        &mut self,
        indent: usize,
        name: &str,
        decorator: &SubroutineVarDecorator,
        number: usize,
    ) -> Result<()> {
        let var_type = match decorator {
            SubroutineVarDecorator::Arg => "arg",
            SubroutineVarDecorator::Local => "local",
        };

        self.var_symbol(indent, var_type, name, number)
    }

    fn var_symbol(
        &mut self,
        indent: usize,
        var_type: &str,
        name: &str,
        number: usize,
    ) -> std::result::Result<(), std::io::Error> {
        self.start(indent, var_type)?;
        self.leaf(indent + 1, "name", name)?;
        self.leaf(indent + 1, "number", &number.to_string())?;

        self.end(indent, var_type)?;
        Ok(())
    }

    fn class_var_dec(&mut self, indent: usize, class_var_dec: ClassVarDecl) -> Result<()> {
        self.start(indent, "classVarDec")?;
        let value = match class_var_dec.decorator {
            ClassVarDecorator::Static => "static",
            ClassVarDecorator::Field => "field",
        };
        self.keyword(indent + 1, value)?;
        self.type_name(indent + 1, &class_var_dec.type_name)?;
        self.variable_list(indent + 1, class_var_dec.declarations)?;
        self.symbol(indent + 1, ";")?;
        self.end(indent, "classVarDec")?;
        Ok(())
    }

    fn subroutine(&mut self, indent: usize, subroutine: Subroutine) -> Result<()> {
        self.start(indent, "subroutineDec")?;
        let value = match subroutine.decorator {
            SubroutineDecorator::Constructor => "constructor",
            SubroutineDecorator::Function => "function",
            SubroutineDecorator::Method => "method",
        };
        self.keyword(indent + 1, value)?;
        match &subroutine.type_name {
            Some(type_name) => self.type_name(indent + 1, type_name)?,
            None => self.keyword(indent + 1, "void")?,
        }

        self.identifier(indent + 1, &subroutine.name)?;
        self.symbol(indent + 1, "(")?;
        self.parameter_list(indent + 1, &subroutine.params)?;
        self.symbol(indent + 1, ")")?;

        self.subroutine_body(indent + 1, subroutine)?;

        self.end(indent, "subroutineDec")?;
        Ok(())
    }

    fn parameter_list(&mut self, indent: usize, params: &Vec<ParamDecl>) -> Result<()> {
        self.start(indent, "parameterList")?;
        let mut first = true;
        for param in params {
            if first {
                first = false;
            } else {
                self.symbol(indent + 1, ",")?;
            }
            self.type_name(indent + 1, &param.type_name)?;
            self.identifier(indent + 1, &param.name)?;
        }
        self.end(indent, "parameterList")?;
        Ok(())
    }

    fn subroutine_body(&mut self, indent: usize, subroutine: Subroutine) -> Result<()> {
        self.start(indent, "subroutineBody")?;
        self.symbol(indent + 1, "{")?;

        for var in subroutine.vars {
            self.var_dec(indent + 1, var)?;
        }

        self.statements(indent + 1, subroutine.statements)?;

        self.symbol(indent + 1, "}")?;
        self.end(indent, "subroutineBody")?;
        Ok(())
    }

    fn var_dec(&mut self, indent: usize, var_dec: SubroutineVarDecl) -> Result<()> {
        self.start(indent, "varDec")?;
        self.keyword(indent + 1, "var")?;
        self.type_name(indent + 1, &var_dec.type_name)?;

        self.variable_list(indent + 1, var_dec.declarations)?;

        self.symbol(indent + 1, ";")?;

        self.end(indent, "varDec")?;
        Ok(())
    }

    fn type_name(&mut self, indent: usize, type_name: &Type) -> Result<()> {
        let (element, value) = match type_name {
            Type::Int => ("keyword", "int".to_string()),
            Type::Char => ("keyword", "char".to_string()),
            Type::Boolean => ("keyword", "boolean".to_string()),
            Type::Class(s) => ("identifier", s.to_string()),
        };
        self.leaf(indent, element, &value)?;
        Ok(())
    }

    fn variable_list(&mut self, indent: usize, vars: Vec<String>) -> Result<()> {
        let mut first = true;
        for var in vars {
            if first {
                first = false;
            } else {
                self.symbol(indent, ",")?;
            }
            self.identifier(indent, &var)?;
        }
        Ok(())
    }

    fn statement_block(&mut self, indent: usize, statements: Vec<Statement>) -> Result<()> {
        self.symbol(indent, "{")?;

        self.statements(indent, statements)?;

        self.symbol(indent + 1, "}")?;
        Ok(())
    }

    fn statements(&mut self, indent: usize, statements: Vec<Statement>) -> Result<()> {
        self.start(indent, "statements")?;
        for statement in statements {
            match statement {
                Statement::Let { name, index, expr } => {
                    self.let_statement(indent + 1, name, index, expr)?
                }
                Statement::If {
                    condition,
                    statements,
                    else_statements,
                } => self.if_statement(indent + 1, condition, statements, else_statements)?,
                Statement::While {
                    condition,
                    statements,
                } => self.while_statement(indent + 1, condition, statements)?,
                Statement::Do { expr } => self.do_statement(indent + 1, expr)?,
                Statement::Return { expr } => self.return_statement(indent + 1, expr)?,
            }
        }

        self.end(indent, "statements")?;
        Ok(())
    }

    fn let_statement(
        &mut self,
        indent: usize,
        name: String,
        index: Option<Expr>,
        expr: Expr,
    ) -> Result<()> {
        self.start(indent, "letStatement")?;

        self.keyword(indent + 1, "let")?;
        self.identifier(indent + 1, &name)?;
        self.indexed_expr(indent + 1, &index)?;

        self.symbol(indent + 1, "=")?;
        self.expr(indent + 1, &expr)?;

        self.symbol(indent + 1, ";")?;

        self.end(indent, "letStatement")?;
        Ok(())
    }

    fn if_statement(
        &mut self,
        indent: usize,
        condition: Expr,
        statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    ) -> Result<()> {
        self.start(indent, "ifStatement")?;

        self.keyword(indent + 1, "if")?;
        self.symbol(indent + 1, "(")?;
        self.expr(indent + 1, &condition)?;
        self.symbol(indent + 1, ")")?;

        self.statement_block(indent + 1, statements)?;

        if let Some(else_statements) = else_statements {
            self.keyword(indent + 1, "else")?;
            self.statement_block(indent + 1, else_statements)?;
        }

        self.end(indent, "ifStatement")?;
        Ok(())
    }

    fn while_statement(
        &mut self,
        indent: usize,
        condition: Expr,
        statements: Vec<Statement>,
    ) -> Result<()> {
        self.start(indent, "whileStatement")?;

        self.keyword(indent + 1, "while")?;
        self.symbol(indent + 1, "(")?;
        self.expr(indent + 1, &condition)?;
        self.symbol(indent + 1, ")")?;

        self.statement_block(indent, statements)?;

        self.end(indent, "whileStatement")?;
        Ok(())
    }

    fn do_statement(&mut self, indent: usize, expr: Expr) -> Result<()> {
        self.start(indent, "doStatement")?;

        self.keyword(indent + 1, "do")?;
        let term = expr.term.as_ref();
        match term {
            Term::SubroutineCall {
                qualifier,
                name,
                exprs,
            } => self.subroutine_call(indent + 1, qualifier, name, exprs)?,
            _ => panic!("got a non subroutine call in do"),
        };

        self.symbol(indent + 1, ";")?;
        self.end(indent, "doStatement")?;
        Ok(())
    }

    fn return_statement(&mut self, indent: usize, expr: Option<Expr>) -> Result<()> {
        self.start(indent, "returnStatement")?;

        self.keyword(indent + 1, "return")?;

        if let Some(expr) = expr {
            self.expr(indent + 1, &expr)?;
        }

        self.symbol(indent + 1, ";")?;
        self.end(indent, "returnStatement")?;
        Ok(())
    }

    fn indexed_expr(&mut self, indent: usize, expr: &Option<Expr>) -> Result<()> {
        if let Some(expr) = expr {
            self.symbol(indent, "[")?;
            self.expr(indent, expr)?;
            self.symbol(indent, "]")?;
        }

        Ok(())
    }

    fn expr(&mut self, indent: usize, expr: &Expr) -> Result<()> {
        self.start(indent, "expression")?;
        self.term(indent + 1, expr.term.as_ref())?;
        for (ref op, ref term) in &expr.ops {
            self.op(indent + 1, op)?;
            self.term(indent + 1, term)?;
        }
        self.end(indent, "expression")?;
        Ok(())
    }

    fn term(&mut self, indent: usize, term: &Term) -> Result<()> {
        self.start(indent, "term")?;
        match term {
            Term::IntegerLit(i) => self.leaf(indent + 1, "integerConstant", &i.to_string())?,
            Term::StringLit(s) => self.leaf(indent + 1, "stringConstant", s)?,
            Term::True => self.keyword(indent + 1, "true")?,
            Term::False => self.keyword(indent + 1, "false")?,
            Term::Null => self.keyword(indent + 1, "null")?,
            Term::This => self.keyword(indent + 1, "this")?,
            Term::Var { name, index } => {
                self.identifier(indent + 1, name)?;
                self.indexed_expr(indent + 1, index)?;
            }
            Term::Bracketed(expr) => {
                self.symbol(indent + 1, "(")?;
                self.expr(indent + 1, expr)?;
                self.symbol(indent + 1, ")")?;
            }
            Term::Unary(unary_op, term) => {
                let symbol = match unary_op {
                    UnaryOp::Neg => "-",
                    UnaryOp::Not => "~",
                };
                self.symbol(indent + 1, symbol)?;
                self.term(indent + 1, term)?;
            }
            Term::SubroutineCall {
                qualifier,
                name,
                exprs,
            } => self.subroutine_call(indent + 1, qualifier, name, exprs)?,
        }

        self.end(indent, "term")?;
        Ok(())
    }

    fn subroutine_call(
        &mut self,
        indent: usize,
        qualifier: &Option<String>,
        name: &str,
        exprs: &[Expr],
    ) -> Result<()> {
        if let Some(qualifier) = qualifier {
            self.identifier(indent, qualifier)?;
            self.symbol(indent, ".")?;
        }
        self.identifier(indent, name)?;
        self.symbol(indent, "(")?;

        self.start(indent, "expressionList")?;

        let mut first = true;
        for expr in exprs {
            if first {
                first = false;
            } else {
                self.symbol(indent + 1, ",")?;
            }
            self.expr(indent + 1, expr)?;
        }
        self.end(indent, "expressionList")?;

        self.symbol(indent, ")")?;
        Ok(())
    }

    fn op(&mut self, indent: usize, op: &Op) -> Result<()> {
        let symbol = match op {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mult => "*",
            Op::Div => "/",
            Op::And => "&",
            Op::Or => "|",
            Op::Lt => "<",
            Op::Gt => ">",
            Op::Eq => "=",
        };
        self.symbol(indent, symbol)?;
        Ok(())
    }

    fn identifier(&mut self, indent: usize, identifier: &str) -> Result<()> {
        self.leaf(indent, "identifier", identifier)?;
        Ok(())
    }

    fn symbol(&mut self, indent: usize, symbol: &str) -> Result<()> {
        self.leaf(indent, "symbol", symbol)?;
        Ok(())
    }

    fn keyword(&mut self, indent: usize, keyword: &str) -> Result<()> {
        self.leaf(indent, "keyword", keyword)?;
        Ok(())
    }
}
