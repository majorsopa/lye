use crate::lexer::literal::Literal;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Literal(Literal),
    Symbol(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Literal(l) => write!(f, "{}", l),
            Token::Symbol(s) => write!(f, "{}", s),
        }
    }
}
