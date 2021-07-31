use crate::parser::node::node_type::NodeType;

#[derive(Debug)]
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

    pub fn add_child(&mut self, child: Self) {
        self.children.push(Box::new(child));
    }
}
