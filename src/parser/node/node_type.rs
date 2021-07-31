use crate::lexer::token::Token;

#[derive(Debug)]
pub enum NodeType {
    Root,

    Token(Token),

    Parentheses,
}