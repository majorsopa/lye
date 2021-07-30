use crate::parser::ast::tree::Tree;
use crate::lexer::token::Token;
use crate::parser::ast::node_type::NodeType;
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

pub struct Parser {
    tokens: Box<dyn Iterator<Item=Token>>,
}

impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            tokens: Box::new(tokens.into_iter()),
        }
    }

    pub fn parse(&mut self) -> Tree {
        let mut open_id = 0;
        let mut last_tree = Tree::new();

        last_tree.new_node(true, NodeType::Expression, open_id, None);

        loop {
            let next_token = self.tokens.next();
            match next_token {
                Some(t) => match &t {
                    Token::Symbol(s) => match s.as_str() {
                        ";" => break, // end of expression
                        "(" => { // start of scope
                            last_tree.add_tree(self.parse());
                            open_id = last_tree.nodes.len();
                        },
                        ")" => break, // end of scope

                        _ => {
                            open_id += 1;

                            let new_node_id = last_tree.new_node(
                                false,
                                NodeType::Token,
                                open_id,
                                Some(t)
                            );
                            last_tree.add_leaf(0, new_node_id);
                        },
                    },

                    _ => {
                        open_id += 1;

                        let new_node_id = last_tree.new_node(
                            false,
                            NodeType::Token,
                            open_id,
                            Some(t)
                        );
                        last_tree.add_leaf(0, new_node_id);
                    },
                },

                None => break, // end of source code
            }
        }

        last_tree
    }
}
