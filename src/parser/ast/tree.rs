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
        let next_open_node_id = NodeId {
            index: open_node_id,
        };

        self.nodes.push(Node {
            children: vec![],

            root,

            node_type,

            data,

            id: next_open_node_id
        });

        next_open_node_id
    }

    pub fn add_leaf(&mut self, parent_id: usize, leaf: NodeId) {
        self.nodes.get_mut(parent_id).unwrap().add_child(leaf);
    }

    pub fn add_tree(&mut self, tree: Tree, parent: NodeId) {
        let mut new_leaf_id = self.nodes.len();
        let mut new_tree = Tree::new();


        let mut first = true;
        for node in tree.nodes {
            if !first {
                let new_node_id = new_tree.new_node(
                    false,
                    NodeType::Token,
                    new_leaf_id,
                    node.data
                );

                // 0 is top of the vector this inner tree. it is not the id.
                new_tree.add_leaf(0, new_node_id);
            } else {
                let new_node_id = new_tree.new_node(
                    false,
                    NodeType::Expression,
                    new_leaf_id,
                    node.data
                );

                self.nodes.get_mut(parent.index).unwrap().add_child(new_node_id);

                first = false
            }

            new_leaf_id += 1;
        }

        for i in 0..new_tree.nodes.len() {
            self.nodes.push(new_tree.nodes.get(i).unwrap().to_owned());
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
