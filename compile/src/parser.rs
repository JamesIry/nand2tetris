use crate::ast::*;

use crate::symbol_table::SymbolTable;
use crate::tokenizer::*;

// parser for Jack. In keeping with the exercise this is a
// standard hand written recursive decent parser rather
// than a higher level parser generator or parser combinators
pub struct Parser<T>
where
    T: Iterator<Item = Result<char, std::io::Error>>,
{
    tokenizer: Tokenizer<T>,
    push_back: Vec<Token>,
}
impl<T> Parser<T>
where
    T: Iterator<Item = Result<char, std::io::Error>>,
{
    pub fn new(tokenizer: Tokenizer<T>) -> Self {
        Self {
            tokenizer,
            push_back: Vec::new(),
        }
    }

    pub fn parse_class(&mut self, symbol_table: &mut SymbolTable) -> Result<Class, ParseError> {
        if !self.check_token(Token::Keyword("class"))? {
            Err(ParseError::MissingClassDeclaration)
        } else {
            let name = self.parse_identifier(ParseError::MissingClassName)?;
            symbol_table.enter_class(name.clone())?;
            self.require_opening_curly(())?;
            let vars = self.parse_class_vars(symbol_table)?;
            let subroutines = self.parse_subroutines(symbol_table)?;
            let class = Class {
                name,
                vars,
                subroutines,
            };
            self.require_closing_curly(())?;
            let token = self.next_token()?;
            match token {
                Some(token) => Err(ParseError::UnexpectedToken(token)),
                None => Ok(class),
            }
        }
    }

    fn parse_class_vars(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Vec<ClassVarDecl>, ParseError> {
        let mut class_vars = Vec::new();
        loop {
            let class_var_decl = self.parse_class_var_decl_opt(symbol_table)?;
            match class_var_decl {
                Some(c) => class_vars.push(c),
                None => break Ok(class_vars),
            }
        }
    }

    fn parse_class_var_decl_opt(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Option<ClassVarDecl>, ParseError> {
        let decorator = self.parse_class_var_decorator_opt()?;
        if let Some(decorator) = decorator {
            let (type_name, declarations) = self.parse_var_declarations()?;
            for name in &declarations {
                symbol_table.enter_class_var(name.clone(), decorator)?;
            }
            Ok(Some(ClassVarDecl {
                decorator,
                type_name,
                declarations,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_class_var_decorator_opt(&mut self) -> Result<Option<ClassVarDecorator>, ParseError> {
        let token = self.next_token()?;
        Ok(match token {
            Some(Token::Keyword("static")) => Some(ClassVarDecorator::Static),
            Some(Token::Keyword("field")) => Some(ClassVarDecorator::Field),
            _ => {
                self.push_back(token);
                None
            }
        })
    }

    fn parse_subroutines(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Vec<Subroutine>, ParseError> {
        let mut subroutines = Vec::new();
        loop {
            let subroutine = self.parse_subroutine_opt(symbol_table)?;
            match subroutine {
                Some(s) => subroutines.push(s),
                None => break Ok(subroutines),
            }
        }
    }

    fn parse_subroutine_opt(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Option<Subroutine>, ParseError> {
        let decorator = self.parse_subroutine_decorator_opt()?;
        if let Some(decorator) = decorator {
            let type_name = if self.check_token(Token::Keyword("void"))? {
                None
            } else {
                Some(self.parse_type()?)
            };
            let name = self.parse_identifier(ParseError::MissingSubroutineName)?;
            symbol_table.enter_subroutine(name.clone(), decorator)?;
            let params = self.parse_params(symbol_table)?;
            self.require_opening_curly(())?;
            let vars = self.parse_subroutine_vars(symbol_table)?;
            let statements = self.parse_statement_list()?;
            self.require_closing_curly(())?;
            Ok(Some(Subroutine {
                decorator,
                type_name,
                name,
                params,
                vars,
                statements,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_params(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Vec<ParamDecl>, ParseError> {
        self.require_opening_paren(())?;
        let mut params = Vec::new();
        if self.check_token(Token::Symbol(')'))? {
            Ok(params)
        } else {
            let param = self.parse_param(symbol_table)?;
            params.push(param);
            loop {
                if !self.check_token(Token::Symbol(','))? {
                    break self.require_closing_paren(params);
                }
                let param = self.parse_param(symbol_table)?;
                params.push(param);
            }
        }
    }

    fn parse_param(&mut self, symbol_table: &mut SymbolTable) -> Result<ParamDecl, ParseError> {
        let type_name = self.parse_type()?;
        let name = self.parse_identifier(ParseError::MissingVariable)?;
        symbol_table.enter_arg(name.clone())?;
        Ok(ParamDecl { type_name, name })
    }

    fn parse_subroutine_decorator_opt(
        &mut self,
    ) -> Result<Option<SubroutineDecorator>, ParseError> {
        let token = self.next_token()?;
        Ok(match token {
            Some(Token::Keyword("constructor")) => Some(SubroutineDecorator::Constructor),
            Some(Token::Keyword("function")) => Some(SubroutineDecorator::Function),
            Some(Token::Keyword("method")) => Some(SubroutineDecorator::Method),
            _ => {
                self.push_back(token);
                None
            }
        })
    }

    fn parse_subroutine_vars(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Vec<SubroutineVarDecl>, ParseError> {
        let mut subroutine_vars = Vec::new();
        loop {
            if !self.check_token(Token::Keyword("var"))? {
                break Ok(subroutine_vars);
            }
            let subroutine_decl = self.parse_subroutine_var_decl(symbol_table)?;
            subroutine_vars.push(subroutine_decl);
        }
    }

    fn parse_subroutine_var_decl(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<SubroutineVarDecl, ParseError> {
        let (type_name, declarations) = self.parse_var_declarations()?;
        for name in &declarations {
            symbol_table.enter_local(name.clone())?;
        }
        Ok(SubroutineVarDecl {
            type_name,
            declarations,
        })
    }

    fn parse_var_declarations(&mut self) -> Result<(Type, Vec<String>), ParseError> {
        let type_name = self.parse_type()?;
        let first_var = self.parse_identifier(ParseError::MissingVariable)?;
        let mut vars = vec![first_var];
        loop {
            if !self.check_token(Token::Symbol(','))? {
                break;
            }
            let var = self.parse_identifier(ParseError::MissingVariable)?;
            vars.push(var);
        }

        self.require_semicolon((type_name, vars))
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let token = self.next_token()?;
        match token {
            Some(Token::Keyword("int")) => Ok(Type::Int),
            Some(Token::Keyword("char")) => Ok(Type::Char),
            Some(Token::Keyword("boolean")) => Ok(Type::Boolean),
            Some(Token::Identifier(s)) => Ok(Type::Class(s)),
            _ => Err(ParseError::MissingType),
        }
    }

    fn parse_statement_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.require_opening_curly(())?;
        let statements = self.parse_statement_list()?;
        self.require_closing_curly(statements)
    }

    fn parse_statement_list(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        loop {
            let statement = self.parse_statement_opt()?;
            match statement {
                Some(statement) => {
                    statements.push(statement);
                }
                None => break Ok(statements),
            }
        }
    }

    fn parse_statement_opt(&mut self) -> Result<Option<Statement>, ParseError> {
        let token = self.next_token()?;
        match token {
            Some(Token::Keyword("let")) => Self::optionalize(self.parse_let_statement()),
            Some(Token::Keyword("if")) => Self::optionalize(self.parse_if_statement()),
            Some(Token::Keyword("while")) => Self::optionalize(self.parse_while_statement()),
            Some(Token::Keyword("do")) => Self::optionalize(self.parse_do_statement()),
            Some(Token::Keyword("return")) => Self::optionalize(self.parse_return_statement()),
            _ => {
                self.push_back(token);
                Ok(None)
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        let name = self.parse_identifier(ParseError::MissingVariable)?;
        let index = {
            if self.check_token(Token::Symbol('['))? {
                let expr = self.parse_expr()?;
                Some(self.require_closing_bracket(expr)?)
            } else {
                None
            }
        };
        self.require(Token::Symbol('='), (), ParseError::MissingEquals)?;
        let expr = self.parse_expr()?;
        let st = Statement::Let { name, index, expr };
        self.require_semicolon(st)
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        let condition = self.parse_bracketed_expr()?;
        let statements = self.parse_statement_block()?;
        let else_statements = if self.check_token(Token::Keyword("else"))? {
            Some(self.parse_statement_block()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            statements,
            else_statements,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        let condition = self.parse_bracketed_expr()?;
        let statements = self.parse_statement_block()?;
        Ok(Statement::While {
            condition,
            statements,
        })
    }

    fn parse_do_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expr()?;
        let term = expr.term.as_ref();
        let ops = &expr.ops;
        match (term, ops) {
            (
                Term::SubroutineCall {
                    qualifier: _,
                    name: _,
                    exprs: _,
                },
                _,
            ) if ops.is_empty() => {
                let st = Statement::Do { expr };
                self.require_semicolon(st)
            }
            _ => Err(ParseError::DoStatementMustBeSubroutineCall),
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expr_opt()?;
        let st = Statement::Return { expr };
        self.require_semicolon(st)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr_opt()?;
        match expr {
            Some(expr) => Ok(expr),
            None => Err(ParseError::MissingExpr),
        }
    }

    fn parse_expr_opt(&mut self) -> Result<Option<Expr>, ParseError> {
        let term = self.parse_term_opt()?;
        match term {
            None => Ok(None),
            Some(term) => {
                let mut ops = Vec::new();

                loop {
                    let token = self.next_token()?;
                    let op = match token {
                        Some(Token::Symbol('+')) => Op::Add,
                        Some(Token::Symbol('-')) => Op::Sub,
                        Some(Token::Symbol('*')) => Op::Mult,
                        Some(Token::Symbol('/')) => Op::Div,
                        Some(Token::Symbol('&')) => Op::And,
                        Some(Token::Symbol('|')) => Op::Or,
                        Some(Token::Symbol('<')) => Op::Lt,
                        Some(Token::Symbol('>')) => Op::Gt,
                        Some(Token::Symbol('=')) => Op::Eq,
                        _ => {
                            self.push_back(token);
                            break Ok(Some(Expr {
                                term: Box::new(term),
                                ops,
                            }));
                        }
                    };
                    let term = self.parse_term()?;
                    ops.push((op, term));
                }
            }
        }
    }

    fn parse_term(&mut self) -> Result<Term, ParseError> {
        let term = self.parse_term_opt()?;
        match term {
            Some(term) => Ok(term),
            None => Err(ParseError::MissingExpr),
        }
    }

    fn parse_term_opt(&mut self) -> Result<Option<Term>, ParseError> {
        let token = self.next_token()?;
        match token {
            Some(Token::IntegerLiteral(n)) => Ok(Some(Term::IntegerLit(n))),
            Some(Token::StringLiteral(s)) => Ok(Some(Term::StringLit(s))),
            Some(Token::Keyword("true")) => Ok(Some(Term::True)),
            Some(Token::Keyword("false")) => Ok(Some(Term::False)),
            Some(Token::Keyword("null")) => Ok(Some(Term::Null)),
            Some(Token::Keyword("this")) => Ok(Some(Term::This)),
            Some(Token::Symbol('-')) => Self::optionalize(self.parse_unary(UnaryOp::Neg)),
            Some(Token::Symbol('~')) => Self::optionalize(self.parse_unary(UnaryOp::Not)),
            Some(Token::Symbol('(')) => Self::optionalize(self.parse_bracketed_term()),
            Some(Token::Identifier(s)) => Self::optionalize(self.parse_var_or_call(s)),
            _ => {
                self.push_back(token);
                Ok(None)
            }
        }
    }

    fn parse_unary(&mut self, op: UnaryOp) -> Result<Term, ParseError> {
        let term = self.parse_term()?;
        Ok(Term::Unary(op, Box::new(term)))
    }

    fn parse_bracketed_term(&mut self) -> Result<Term, ParseError> {
        self.push_back(Some(Token::Symbol('(')));
        let expr = self.parse_bracketed_expr()?;
        Ok(Term::Bracketed(expr))
    }

    fn parse_bracketed_expr(&mut self) -> Result<Expr, ParseError> {
        self.require_opening_paren(())?;
        let expr = self.parse_expr()?;
        self.require_closing_paren(expr)
    }

    fn parse_var_or_call(&mut self, s: String) -> Result<Term, ParseError> {
        let token = self.next_token()?;
        match token {
            Some(Token::Symbol('.')) => self.parse_qualified_subroutine_call(s),
            Some(Token::Symbol('(')) => self.parse_subroutine_call(None, s),
            Some(Token::Symbol('[')) => self.parse_indexed_var_term(s),
            _ => {
                self.push_back(token);
                Ok(Term::Var {
                    name: s,
                    index: None,
                })
            }
        }
    }

    fn parse_qualified_subroutine_call(&mut self, qualifier: String) -> Result<Term, ParseError> {
        let name = self.parse_identifier(ParseError::MissingSubroutineName)?;
        self.require_opening_paren(())?;
        self.parse_subroutine_call(Some(qualifier), name)
    }

    fn parse_subroutine_call(
        &mut self,
        qualifier: Option<String>,
        name: String,
    ) -> Result<Term, ParseError> {
        let exprs = self.parse_expr_list()?;
        let term = Term::SubroutineCall {
            qualifier,
            name,
            exprs,
        };
        self.require_closing_paren(term)
    }

    fn parse_indexed_var_term(&mut self, var: String) -> Result<Term, ParseError> {
        let expr = self.parse_index_expr()?;
        Ok(Term::Var {
            name: var,
            index: Some(expr),
        })
    }

    fn parse_index_expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;
        self.require_closing_bracket(expr)
    }

    fn parse_identifier(&mut self, error: ParseError) -> Result<String, ParseError> {
        let token = self.next_token()?;
        match token {
            Some(Token::Identifier(s)) => Ok(s),
            _ => Err(error),
        }
    }

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut exprs = Vec::new();

        let expr = self.parse_expr_opt()?;
        match expr {
            Some(expr) => {
                exprs.push(expr);
                loop {
                    if self.check_token(Token::Symbol(','))? {
                        let expr = self.parse_expr()?;
                        exprs.push(expr);
                    } else {
                        break Ok(exprs);
                    }
                }
            }

            None => Ok(exprs),
        }
    }

    fn next_token(&mut self) -> Result<Option<Token>, ParseError> {
        if !self.push_back.is_empty() {
            Ok(self.push_back.pop())
        } else {
            let token = self.tokenizer.next();
            match token {
                None => Ok(None),
                Some(Ok(token)) => Ok(Some(token)),
                Some(Err(token_error)) => Err(ParseError::TokenError(token_error)),
            }
        }
    }

    fn check_token(&mut self, target: Token) -> Result<bool, ParseError> {
        let token = self.next_token()?;
        match token {
            Some(token) if token == target => Ok(true),
            _ => {
                self.push_back(token);
                Ok(false)
            }
        }
    }

    fn push_back(&mut self, token: Option<Token>) {
        if let Some(token) = token {
            self.push_back.push(token)
        }
    }

    fn optionalize<X>(result: Result<X, ParseError>) -> Result<Option<X>, ParseError> {
        result.map(|x| Some(x))
    }

    fn require_opening_curly<X>(&mut self, ok: X) -> Result<X, ParseError> {
        self.require(Token::Symbol('{'), ok, ParseError::MissingOpeningCurly)
    }

    fn require_closing_curly<X>(&mut self, ok: X) -> Result<X, ParseError> {
        self.require(Token::Symbol('}'), ok, ParseError::MissingClosingCurly)
    }

    fn require_closing_bracket<X>(&mut self, ok: X) -> Result<X, ParseError> {
        self.require(Token::Symbol(']'), ok, ParseError::MissingClosingBracket)
    }

    fn require_opening_paren<X>(&mut self, ok: X) -> Result<X, ParseError> {
        self.require(Token::Symbol('('), ok, ParseError::MissingOpeningParen)
    }

    fn require_closing_paren<X>(&mut self, ok: X) -> Result<X, ParseError> {
        self.require(Token::Symbol(')'), ok, ParseError::MissingClosingParen)
    }

    fn require_semicolon<X>(&mut self, ok: X) -> Result<X, ParseError> {
        self.require(Token::Symbol(';'), ok, ParseError::MissingSemicolon)
    }

    fn require<X>(&mut self, expected: Token, ok: X, error: ParseError) -> Result<X, ParseError> {
        let token = self.next_token()?;
        if token == Some(expected) {
            Ok(ok)
        } else {
            Err(error)
        }
    }
}
