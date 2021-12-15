use std::fmt::{Debug, Formatter};

enum Literal {
    Str(String),
    Int(i32),
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        return match self {
            Literal::Str(lit_str) => lit_str.clone(),
            Literal::Int(lit_int) => lit_int.to_string(),
        };
    }
}

impl Literal {
    pub fn from_i32(in_i32: i32) -> Self {
        Self::Int(in_i32)
    }

    pub fn from_str(in_string: String) -> Self {
        Self::Str(in_string)
    }

    pub fn to_pretty_string(&self) -> String {
        return match self {
            Literal::Str(_) => format!("String Literal {{ {} }}", self.to_string()),
            Literal::Int(_) => format!("i32 Literal {{ {} }}", self.to_string()),
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pretty_string())
    }
}



pub enum Token {
    Literal(Literal),
    Lexeme(String),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return match self {
            Token::Literal(lit) => lit.to_string(),
            Token::Lexeme(lexeme) => lexeme.clone(),
        }
    }
}

impl Token {
    pub fn from_i32(in_i32: i32) -> Self {
        Self::Literal(Literal::Int(in_i32))
    }

    pub fn new_string_literal(in_string: String) -> Self {
        Self::Literal(Literal::Str(in_string))
    }

    pub fn new_lexeme(in_string: String) -> Self {
        Self::Lexeme(in_string)
    }

    pub fn to_pretty_string(&self) -> String {
        return match self {
            Token::Literal(literal) => format!("Literal {{ {} }},", literal.to_pretty_string()),
            Token::Lexeme(lexeme) => format!("Lexeme {{ {} }},", lexeme),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pretty_string())
    }
}
