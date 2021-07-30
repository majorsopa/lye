use crate::parser::node::link::Link;
use crate::parser::node::node_type::NodeType;

pub struct Node {
    pub(crate) elem: NodeType,
    pub(crate) next: Link,
}