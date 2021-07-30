use crate::parser::node::node::Node;

pub enum Link {
    Empty,
    More(Box<Node>),
}
