use crate::lexer::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: Vec<NodeId>,

    pub(crate) data: Option<Token>,

    pub(crate) id: NodeId,
}

impl Node {
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn add_parent(&mut self, parent: NodeId) {
        self.parent = Some(parent);
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

#[derive(Clone, Copy, Debug)]
pub struct NodeId {
    pub(crate) index: usize,
}

impl NodeId {
    pub fn add_value(&mut self, amount: usize) {
        self.index += amount;
    }

    pub fn sub_value(&mut self, amount: usize) {
        self.index -= amount;
    }

    pub fn set_value(&mut self, value: usize) {
        self.index = value;
    }
}
