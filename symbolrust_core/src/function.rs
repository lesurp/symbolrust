use crate::node::Node;

#[derive(Debug, Clone, Copy)]
pub enum FunctionError {
    WrongNumberArguments,
}

pub type FunctionResult = Result<Node, FunctionError>;
impl From<Node> for FunctionResult {
    fn from(n: Node) -> FunctionResult {
        Ok(n)
    }
}

impl From<FunctionError> for FunctionResult {
    fn from(e: FunctionError) -> FunctionResult {
        Err(e)
    }
}
// TODO: impl this for binary operators?
// Not required rn as our CLI already handles precedence and continuosly concatenantes nodes,
// but to be more general additions, multiplications, should implement this (and virutally everything else...)
pub trait Function {
    fn from_args(args: Vec<Node>) -> FunctionResult;
    fn description() -> &'static str;
}
