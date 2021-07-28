use crate::parser::ast::tree::Tree;
use crate::lexer::token::Token;
use crate::parser::ast::node_type::NodeType;
use crate::parser::ast::node_id::NodeId;

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
        self.parse_tokens(0)
    }

    fn parse_tokens(&mut self, mut open_id: usize) -> Tree {
        let mut last_tree = Tree::new();

        let root_id = last_tree.new_node(true, NodeType::Expression, open_id, None);

        loop {
            let next_token = self.tokens.next();
            match next_token {
                Some(t) => match &t {
                    Token::Symbol(s) => match s.as_str() {
                        ";" => break, // end of expression
                        "(" => { // start of scope
                            open_id += 1;


                            let inner_tree = self.parse_tokens(0);

                            open_id += inner_tree.nodes.len() - 1;

                            last_tree.add_tree(inner_tree, root_id);
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
                            last_tree.add_leaf(root_id.index, new_node_id);
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
                        last_tree.add_leaf(root_id.index, new_node_id);
                    },
                },

                None => break, // end of source code
            }
        }

        last_tree
    }
}
