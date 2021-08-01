use crate::lexer::token::Token;
use crate::parser::node::tree::Tree;
use crate::parser::node::node_type::NodeType;
use std::vec::IntoIter;
use std::iter::Peekable;
use crate::parser::node::node::Node;
use std::borrow::Borrow;
use crate::lexer::literal::Literal;
use crate::lexer::tokenizer::BINARY_OPERATORS;


macro_rules! prefix_grammar_check {
    ($input:expr, $binary_operators:expr) => {
        {
            let mut ret_bool = false;
            for symbol in $binary_operators {
                if $input[0] == Token::Symbol(symbol.parse().unwrap()) {
                    ret_bool = true;
                }
            }
            ret_bool
        }
    }
}

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
        let mut start_tree = Tree::new(Node::new(NodeType::Null));
        //todo determine if its even a binary expression or what
        self.parse_binary_expressions(start_tree)
    }


    fn parse_binary_expressions(&mut self, mut ret_tree: Tree) -> Tree { // returns a BinaryExpression Tree

        match self.tokens.peek() {
            Some(token) => match token {
                Token::Symbol(symbol) if symbol.as_str() == ";" => {
                    self.tokens.next(); // consume the semicolon iterator
                    ()
                },
                _ => ()
            },
            None => return ret_tree,
        }

        {
            let mut prefix_binary_expression: [Token; 3] = [
                Token::Literal(Literal::Str("if you see this in your code, you did something very wrong".parse().unwrap())),  // no copy for token
                Token::Literal(Literal::Str("if you see this in your code, you did something very wrong".parse().unwrap())),  // no copy for token
                Token::Literal(Literal::Str("if you see this in your code, you did something very wrong".parse().unwrap())),  // no copy for token
            ];

            {
                let mut first_terminal_found = false;
                for _i in 0..3 {
                    let next_token = self.tokens.next();
                    match next_token.clone() {
                        Some(token) => match &token {
                            Token::Symbol(token_symbol) => if BINARY_OPERATORS.contains(&token_symbol.as_str()) {
                                prefix_binary_expression[0] = next_token.unwrap();
                            } else {
                                panic!("`{}` is not a valid binary operator.", token_symbol);
                            },

                            _ => if !first_terminal_found {
                                prefix_binary_expression[1] = next_token.unwrap();
                                first_terminal_found = true;
                            } else {
                                prefix_binary_expression[2] = next_token.unwrap();
                            },
                        }
                        None => return ret_tree,
                    }
                }
            }
            // check grammar is correct
            println!("{:?}", prefix_binary_expression);
            assert!(prefix_grammar_check!(prefix_binary_expression, BINARY_OPERATORS));

            let mut binary_expression_node_tree = Tree::new(
                Node::new(NodeType::Null)
            );
            let mut binary_expression_tree = Tree::new(
                Node::new(NodeType::Null)
            );
            binary_expression_node_tree.add_node(Box::new(Node::new(NodeType::Token(prefix_binary_expression[0].clone()))));
            binary_expression_tree.add_node(Box::new(Node::new(NodeType::Token(prefix_binary_expression[1].clone()))));
            binary_expression_tree.add_node(Box::new(Node::new(NodeType::Token(prefix_binary_expression[2].clone()))));
            binary_expression_node_tree.graft(binary_expression_tree, NodeType::Null);
            ret_tree.graft(binary_expression_node_tree, NodeType::BinaryExpression);
        }

        Tree::new(
            Node::new(NodeType::Null)
        ).graft(
            self.parse_binary_expressions(
                ret_tree
            ), NodeType::Root
        )
    }
}
