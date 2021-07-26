use crate::parser::ast::node::{Node, NodeId};
use crate::lexer::tokenizer::Token;
use std::ops::Deref;

macro_rules! println_debug {
    ($input:expr, $id:expr) => {
        println!("[DEBUG {}] {:#?}", $id.to_string(), $input);
    }
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    current_node: NodeId,
}

impl Iterator for Tree {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.nodes.get(self.current_node.index + 1) {
            None => None,
            Some(node) => {
                self.current_node.add_value(1);
                Some(node.clone())
            },
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: vec![],
            current_node: NodeId {
                index: 0
            }
        }
    }

    pub fn new_node(&mut self) -> NodeId {
        let next_open_node_id = NodeId {
            index: self.nodes.len(),
        };

        self.nodes.push(Node {
            parent: None,
            children: vec![],

            data: None,

            id: next_open_node_id
        });

        next_open_node_id
    }

    pub fn add_leaf(&mut self, parent_id: NodeId, leaf_id: NodeId, data: Token) {
        self.nodes.get_mut(parent_id.index).unwrap().add_child(leaf_id);

        let leaf_node = self.nodes.get_mut(leaf_id.index).unwrap();

        leaf_node.add_parent(parent_id);
        leaf_node.set_data(data);
    }

    pub fn add_tree(&mut self, tree: Tree, parent: NodeId) {
        let main_tree_size = self.nodes.len();
        for mut node in tree {
            node.add_id(main_tree_size);
        }

        let leaf_id = NodeId { index: main_tree_size - 1 };


        self.nodes.get_mut(parent.index).unwrap().add_child(leaf_id);

        let leaf_node = self.nodes.get_mut(leaf_id.index).unwrap();

        leaf_node.add_parent(parent);
    }
}
