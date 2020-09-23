use crate::node::Node;

pub trait Function {
    fn from_args(args: Vec<Node>) -> Result<Node, ()>;
}
