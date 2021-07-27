use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum NodeType {
    Expression,
    Token,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Expression => write!(f, "Expression"),
            NodeType::Token => write!(f, "Token"),
        }
    }
}
