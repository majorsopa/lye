use std::fmt::{Display, Formatter};

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

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index.to_string())
    }
}
