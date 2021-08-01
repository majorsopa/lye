use crate::parser::node::node_type::NodeType;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) node_type: NodeType,
    pub(crate) children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child_box: Box<Node>) -> Node {
        self.children.push(child_box.clone());
        *child_box
    }
}
