use crate::node::Node;

pub enum FunctionError {
    WrongNumberArguments,
}

// TODO: impl this for binary operators?
// Not required rn as our CLI already handles precedence and continuosly concatenantes nodes,
// but to be more general additions, multiplications, should implement this (and virutally everything else...)
pub trait Function {
    fn from_args(args: Vec<Node>) -> Result<Node, FunctionError>;
    fn description() -> &'static str;
}
