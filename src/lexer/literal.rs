use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Str(String),
    Integer(i32),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Integer(i) => write!(f, "{}", i.to_string()),
        }
    }
}
