use std::mem;
use crate::parser::node::node::Node;
use crate::parser::node::node_type::NodeType;

#[derive(Debug, Clone)]
pub struct Tree {
    pub(crate) root: Box<Node>,
}

impl Tree {
    pub fn new(node: Node) -> Self {
        Self { root: Box::new(node) }
    }

    pub fn add_node(&mut self, node_box_to_add: Box<Node>) -> Node {
        self.root.add_child(node_box_to_add)
    }

    pub fn graft(&mut self, mut tree: Tree) -> Tree {
        tree.root.node_type = NodeType::BinaryExpression;
        self.root.add_child(tree.root.clone());
        tree
    }
}
