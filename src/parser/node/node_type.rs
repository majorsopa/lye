use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum NodeType {
    Undecided,

    Root,
    BinaryExpression,

    Terminal(Token),
    Token(Token),
}