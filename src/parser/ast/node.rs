use crate::lexer::tokenizer::Token;
use crate::parser::ast::node_id::NodeId;
use crate::parser::ast::node_type::NodeType;
use std::fmt::{Display, Formatter};


#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) children: Vec<NodeId>,

    pub(crate) root: bool,

    pub(crate) node_type: NodeType,

    pub(crate) data: Option<Token>,

    pub(crate) id: NodeId,
}

impl Node {
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn set_data(&mut self, token: Token) {
        self.data = Some(token);
    }

    pub fn add_id(&mut self, amount: usize) {
        self.id.add_value(amount);
    }

    pub fn sub_id(&mut self, amount: usize) {
        self.id.sub_value(amount);
    }

    pub fn set_id(&mut self, amount: usize) {
        self.id.set_value(amount);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut root_bool_string = String::new();
        if self.root {
            root_bool_string.push_str("true");
        } else {
            root_bool_string.push_str("false");
        }

        let mut children_string = String::from('[');
        for child in self.children {
            children_string.push_str(&*format!("{}, ", child.index.to_string()))
        }
        // get rid of the space and comma at the end
        children_string.pop();
        children_string.pop();
        children_string.push(']');


        write!(
            f,
            "Root: {root}\nChildren: {children_vec}\n",
            root = root_bool_string,
            children_vec = children_string,
        )
    }
}
