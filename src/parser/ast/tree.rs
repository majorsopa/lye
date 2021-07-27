use crate::parser::ast::node::{Node, NodeId, NodeType};
use crate::lexer::tokenizer::Token;
use std::ops::Deref;
use std::fmt::{Display, Formatter};

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
        let expr_node_id = new_leaf_id;
        let mut new_tree = Tree::new();


        let mut first = true;
        for node in tree.nodes {
            let new_node_id = new_tree.new_node(
                false,
                NodeType::Token,
                new_leaf_id,
                node.data
            );

            new_leaf_id += 1;

            if !first {
                // 0 is top of the vector this inner tree. it is not the id.
                new_tree.add_leaf(0, new_node_id);
            } else {
                first = false
            }
        }



        self.nodes.get_mut(parent.index).unwrap().add_child(NodeId { index: expr_node_id });

        for i in 0..new_tree.nodes.len() {
            self.nodes.push(new_tree.nodes.get(i).unwrap().to_owned());
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut final_string = String::new();

        for node in self.nodes {

        }


        write!(f, "hi")
    }
}
