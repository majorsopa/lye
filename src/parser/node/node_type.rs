use crate::lexer::token::Token;

pub enum NodeType {
    Token(Token),
    Expression,
}