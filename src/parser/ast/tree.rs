use crate::parser::ast::node::Node;
use std::fmt::{Display, Formatter};
use crate::lexer::token::Token;
use crate::parser::ast::node_id::NodeId;
use crate::parser::ast::node_type::NodeType;

#[derive(Debug, Clone)]
pub struct Tree {
    pub(crate) nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: vec![],
        }
    }

    pub fn new_node(&mut self, root: bool, node_type: NodeType, open_node_id: usize, data: Option<Token>) -> NodeId {
        let id = NodeId {
            index: open_node_id,
        };

        self.nodes.push(Node {
            children: vec![],

            root,

            node_type,

            data,

            id
        });

        id
    }

    pub fn add_leaf(&mut self, parent_id: usize, leaf: NodeId) {
        self.nodes.get_mut(parent_id).unwrap().add_child(leaf);
    }

    pub fn add_tree(&mut self, tree: Tree) {
        let mut new_leaf_id = self.nodes.len();
        let mut new_tree = Tree::new();


        {
            let new_node_id = new_tree.new_node(
                false,
                NodeType::Expression,
                new_leaf_id,
                None,
            );
            self.nodes.push(new_tree.nodes.get(0).unwrap().to_owned());
            self.nodes.get_mut(0).unwrap().add_child(new_node_id);

            new_leaf_id += 1;
        }

        for i in 1..tree.nodes.len() {
            let new_node_id = new_tree.new_node(
                false,
                NodeType::Token,
                new_leaf_id,
                tree.nodes.get(i).unwrap().data.clone()
            );

            self.nodes.push(new_tree.nodes.get(i).unwrap().to_owned());
            // 0 is top of the vector this inner tree. it is not the id.
            new_tree.add_leaf(0, new_node_id);

            new_leaf_id += 1;
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut node_vec_string = String::from("[\n");

        for node in &self.nodes {
            node_vec_string.push('\t');
            node_vec_string.push_str(node.to_string().as_str())
        }
        // get rid of the comma and newline at the end
        node_vec_string.pop();
        node_vec_string.push_str("\n]");


        write!(f, "{node_vec}", node_vec = node_vec_string)
    }
}
