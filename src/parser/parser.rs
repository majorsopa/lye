use crate::lexer::token::Token;
use crate::parser::node::tree::Tree;
use crate::parser::node::node_type::NodeType;
use std::vec::IntoIter;
use std::iter::Peekable;


pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Tree {
        let mut root_list = Tree::new(NodeType::Root);


        loop {
            match self.tokens.next() {
                Some(token) => match &token {
                    Token::Literal(_) => root_list.add_node(NodeType::Token(token)),
                    Token::Symbol(token_symbol) => match token_symbol.as_str() {
                        "(" => root_list.add_list(self.parse()), // recursively parse nested parentheses
                        ")" => break, // end of parentheses, break out of recursion
                        _ => root_list.add_node(NodeType::Token(token)),
                    }
                }

                None => break, // end of lye source code
            }
        }
        root_list
    }
}
