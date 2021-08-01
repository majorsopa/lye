use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum NodeType {
    NullDebug(String), // string for debugging
    Null,

    Root,
    BinaryExpression,

    Terminal(Token),
    Token(Token),
}