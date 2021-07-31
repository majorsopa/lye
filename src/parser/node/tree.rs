use std::mem;
use crate::parser::node::node::Node;
use crate::parser::node::node_type::NodeType;

#[derive(Debug)]
pub struct Tree {
    root: Box<Node>,
}

impl Tree {
    pub fn new(node_type: NodeType) -> Self {
        let new_node = Box::new(Node::new(node_type));
        Self { root: new_node }
    }

    pub fn add_node(&mut self, node_type: NodeType) {
        let node_to_add = Node::new(node_type);
        self.root.add_child(node_to_add);
    }

    pub fn add_list(&mut self, mut list: Self) {
        list.root.node_type = NodeType::Parentheses;
        self.root.add_child(*list.root);
    }
}
