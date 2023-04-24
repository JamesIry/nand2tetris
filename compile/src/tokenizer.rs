// tokenizer for jack. in the spirit the exercise I decided to hand write
// rather than use a scanner generator or something moderately high level
// like regular expressions or even parser combinators.
// the result isn't terrible, a pretty standard hand written state machine.
// however, I made things a little(lot) simpler by keeping one charater
// of push_back and by not distinguishing identifiers from keywords
// until it's ready to emit a token. These two things greatly reduce
// the number of states the tokenizer can be in.

use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    IntegerLiteral(u16),
    StringLiteral(String),
    Identifier(String),
    Symbol(char),
    Keyword(&'static str),
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("A comment was started but not closed before the end of the file \n--\"{0}\"")]
    UnclosedComment(String),
    #[error("A string was started but not closed before the end of the file \n--\"{0}\"")]
    UnclosedString(String),
    #[error("Identifier was invalid \"{0}\"")]
    InvalidIdentifier(String),
    #[error("Was out of range (0..32767) \"{0}\"")]
    IntegerOutOfRange(String),
    #[error("There was an IO error \n--\"{0}\"")]
    IOError(std::io::Error),
}

pub struct Tokenizer<T>
where
    T: Iterator<Item = Result<char, std::io::Error>>,
{
    iter: T,
    accum: String,
    state: State,
    push_back: Option<Category>,
}

impl<T> Tokenizer<T>
where
    T: Iterator<Item = Result<char, std::io::Error>>,
{
    pub fn new(iter: T) -> Self {
        Self {
            iter,
            state: State::Fresh,
            accum: String::new(),
            push_back: None,
        }
    }

    fn next_token(&mut self) -> Option<Result<Token, TokenError>> {
        loop {
            if self.state == State::Eof {
                break None;
            }

            match self.next_category() {
                Err(error) => {
                    break Some(Err(TokenError::IOError(error)));
                }
                Ok(next) => {
                    let opt_token = match self.state {
                        State::Fresh => self.handle_fresh(next),
                        State::PossibleCommentStart(depth) => {
                            self.handle_possible_comment_start(next, depth)
                        }
                        State::BlockComment(depth) => self.handle_block_comment(next, depth),
                        State::PossibleCommentEnd(depth) => {
                            self.handle_possible_comment_end(next, depth)
                        }
                        State::LineComment => self.handle_line_comment(next),
                        State::IdentifierOrKeyword => self.handle_identifier_or_keyword(next),
                        State::InvalidIdentifier => self.handle_invalid_identifier(next),
                        State::Integer => self.handle_integer(next),
                        State::Eof => self.handle_eof(next),
                        State::String => self.handle_string(next),
                    };

                    if let Some(token) = opt_token {
                        break Some(token);
                    }
                }
            }
        }
    }

    fn handle_eof(&mut self, next: Category) -> Option<Result<Token, TokenError>> {
        self.transition(State::Eof, Action::PushBack(next), Self::no_token)
    }

    fn handle_fresh(&mut self, next: Category) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Letter(c) => {
                self.transition(State::IdentifierOrKeyword, Action::Accum(c), Self::no_token)
            }
            Category::OtherCharacter(c) => {
                self.transition(State::InvalidIdentifier, Action::Accum(c), Self::no_token)
            }
            Category::Underscore => self.transition(
                State::IdentifierOrKeyword,
                Action::Accum('_'),
                Self::no_token,
            ),
            Category::Number(c) => {
                self.transition(State::Integer, Action::Accum(c), Self::no_token)
            }
            Category::Symbol('/') => self.transition(
                State::PossibleCommentStart(0),
                Action::Ignore,
                Self::no_token,
            ),
            Category::Symbol(c) => {
                self.transition(State::Fresh, Action::Ignore, |_| Some(Ok(Token::Symbol(c))))
            }
            Category::Quote => self.transition(State::String, Action::Ignore, Self::no_token),
            Category::WhiteSpace(_) => {
                self.transition(State::Fresh, Action::Ignore, Self::no_token)
            }
            Category::Eof => self.transition(State::Eof, Action::PushBack(next), Self::no_token),
        }
    }

    fn handle_possible_comment_start(
        &mut self,
        next: Category,
        depth: usize,
    ) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Symbol('*') => self.transition(
                State::BlockComment(depth + 1),
                Action::Ignore,
                Self::no_token,
            ),
            Category::Symbol('/') => {
                self.transition(State::LineComment, Action::Ignore, Self::no_token)
            }
            _ => self.transition(State::Fresh, Action::PushBack(next), |_| {
                Some(Ok(Token::Symbol('/')))
            }),
        }
    }

    fn handle_block_comment(
        &mut self,
        next: Category,
        depth: usize,
    ) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Symbol('*') => self.transition(
                State::PossibleCommentEnd(depth),
                Action::Ignore,
                Self::no_token,
            ),
            Category::Eof => self.transition(State::Fresh, Action::PushBack(next), Self::no_token),
            _ => self.transition(
                State::BlockComment(depth),
                Action::Accum(next.character()),
                Self::no_token,
            ),
        }
    }

    fn handle_possible_comment_end(
        &mut self,
        next: Category,
        depth: usize,
    ) -> Option<Result<Token, TokenError>> {
        match (next, depth) {
            (Category::Symbol('/'), 1) => {
                self.transition(State::Fresh, Action::Ignore, Self::no_token)
            }
            (Category::Symbol('/'), _) => self.transition(
                State::BlockComment(depth - 1),
                Action::Ignore,
                Self::no_token,
            ),
            (Category::Symbol('*'), _) => self.transition(
                State::PossibleCommentEnd(depth),
                Action::Accum('*'),
                Self::no_token,
            ),
            (Category::Eof, _) => self.transition(
                State::Fresh,
                Action::PushBack(next),
                Self::token_error(TokenError::UnclosedComment),
            ),
            _ => self.transition(
                State::BlockComment(depth),
                Action::Accum(next.character()),
                Self::no_token,
            ),
        }
    }

    fn handle_line_comment(&mut self, next: Category) -> Option<Result<Token, TokenError>> {
        match next {
            Category::WhiteSpace(c) if c == '\r' || c == '\n' => {
                self.transition(State::Fresh, Action::Ignore, Self::no_token)
            }
            Category::Eof => self.transition(State::Fresh, Action::PushBack(next), Self::no_token),
            _ => self.transition(State::LineComment, Action::Ignore, Self::no_token),
        }
    }

    fn handle_identifier_or_keyword(
        &mut self,
        next: Category,
    ) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Letter(c) => {
                self.transition(State::IdentifierOrKeyword, Action::Accum(c), Self::no_token)
            }
            Category::Number(c) => {
                self.transition(State::IdentifierOrKeyword, Action::Accum(c), Self::no_token)
            }
            Category::Underscore => self.transition(
                State::IdentifierOrKeyword,
                Action::Accum('_'),
                Self::no_token,
            ),
            Category::OtherCharacter(c) => {
                self.transition(State::InvalidIdentifier, Action::Accum(c), Self::no_token)
            }
            _ => self.transition(
                State::Fresh,
                Action::PushBack(next),
                Self::identifier_or_keyword_token,
            ),
        }
    }

    fn handle_invalid_identifier(&mut self, next: Category) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Letter(c) => {
                self.transition(State::InvalidIdentifier, Action::Accum(c), Self::no_token)
            }
            Category::Number(c) => {
                self.transition(State::InvalidIdentifier, Action::Accum(c), Self::no_token)
            }
            Category::Underscore => {
                self.transition(State::InvalidIdentifier, Action::Accum('_'), Self::no_token)
            }
            Category::OtherCharacter(c) => {
                self.transition(State::InvalidIdentifier, Action::Accum(c), Self::no_token)
            }
            _ => self.transition(
                State::Fresh,
                Action::PushBack(next),
                Self::token_error(TokenError::InvalidIdentifier),
            ),
        }
    }

    fn handle_integer(&mut self, next: Category) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Number(c) => {
                self.transition(State::Integer, Action::Accum(c), Self::no_token)
            }
            _ => self.transition(State::Fresh, Action::PushBack(next), Self::integer_token),
        }
    }

    fn handle_string(&mut self, next: Category) -> Option<Result<Token, TokenError>> {
        match next {
            Category::Quote => self.transition(State::Fresh, Action::Ignore, |s| {
                Some(Ok(Token::StringLiteral(s)))
            }),
            Category::Eof => self.transition(
                State::Fresh,
                Action::Ignore,
                Self::token_error(TokenError::UnclosedString),
            ),
            _ => self.transition(
                State::String,
                Action::Accum(next.character()),
                Self::no_token,
            ),
        }
    }

    fn no_token(_: String) -> Option<Result<Token, TokenError>> {
        None
    }

    fn token_error(
        f: impl Fn(String) -> TokenError,
    ) -> impl Fn(String) -> Option<Result<Token, TokenError>> {
        move |s| Some(Err(f(s)))
    }

    fn identifier_or_keyword_token(s: String) -> Option<Result<Token, TokenError>> {
        static KEYWORDS: [&str; 21] = [
            "boolean",
            "char",
            "class",
            "constructor",
            "do",
            "else",
            "false",
            "field",
            "function",
            "if",
            "int",
            "let",
            "method",
            "null",
            "return",
            "static",
            "this",
            "true",
            "var",
            "void",
            "while",
        ];

        KEYWORDS
            .binary_search(&s.as_str())
            .map(|index| Some(Ok(Token::Keyword(KEYWORDS[index]))))
            .unwrap_or_else(|_| Some(Ok(Token::Identifier(s))))
    }

    fn integer_token(s: String) -> Option<Result<Token, TokenError>> {
        if s.len() <= 5 {
            if let Ok(i) = s.parse::<u16>() {
                if i <= 32767 {
                    return Some(Ok(Token::IntegerLiteral(i)));
                }
            }
        }
        Some(Err(TokenError::IntegerOutOfRange(s)))
    }

    fn next_category(&mut self) -> Result<Category, std::io::Error> {
        static SYMBOLS: [char; 19] = [
            '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '<', '&', '>', '|', '=',
            '~', '/',
        ];

        if let Some(back) = self.push_back.take() {
            Ok(back)
        } else {
            match self.iter.next() {
                Some(Ok(c)) => Ok({
                    if c.is_whitespace() {
                        Category::WhiteSpace(c)
                    } else if c == '"' {
                        Category::Quote
                    } else if c == '_' {
                        Category::Underscore
                    } else if SYMBOLS.contains(&c) {
                        Category::Symbol(c)
                    } else if c.is_numeric() {
                        Category::Number(c)
                    } else if c.is_alphabetic() {
                        Category::Letter(c)
                    } else {
                        Category::OtherCharacter(c)
                    }
                }),
                Some(Err(io_error)) => Err(io_error),

                None => Ok(Category::Eof),
            }
        }
    }

    fn transition(
        &mut self,
        state: State,
        action: Action,
        f: impl Fn(String) -> Option<Result<Token, TokenError>>,
    ) -> Option<Result<Token, TokenError>> {
        self.state = state;
        let result = f(self.accum.clone());
        match action {
            Action::PushBack(next) => {
                assert!(self.push_back.is_none());
                self.accum.clear();
                self.push_back = Some(next);
            }
            Action::Accum(c) => self.accum.push(c),
            Action::Ignore => self.accum.clear(),
        }
        result
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Action {
    PushBack(Category),
    Accum(char),
    Ignore,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Category {
    Letter(char),
    OtherCharacter(char),
    Number(char),
    Symbol(char),
    WhiteSpace(char),
    Eof,
    Quote,
    Underscore,
}
impl Category {
    fn character(&self) -> char {
        match self {
            Category::Letter(c) => *c,
            Category::OtherCharacter(c) => *c,
            Category::Number(c) => *c,
            Category::Symbol(c) => *c,
            Category::WhiteSpace(c) => *c,
            Category::Eof => 0 as char,
            Category::Quote => '"',
            Category::Underscore => '_',
        }
    }
}

impl<T> Iterator for Tokenizer<T>
where
    T: Iterator<Item = Result<char, std::io::Error>>,
{
    type Item = Result<Token, TokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum State {
    Fresh,
    PossibleCommentStart(usize),
    PossibleCommentEnd(usize),
    LineComment,
    BlockComment(usize),
    IdentifierOrKeyword,
    Integer,
    String,
    Eof,
    InvalidIdentifier,
}
