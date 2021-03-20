use crate::function::{Function, FunctionError, FunctionResult};
use crate::node::Node;
use crate::ops::Power;

/// TODO: make some generic Binary struct so we can more easily implement visitors
#[derive(Clone, Debug, PartialEq)]
pub struct Exponential;

impl Function for Exponential {
    fn from_args(mut args: Vec<Node>) -> FunctionResult {
        match args.len() {
            1 => Ok(Power::exp(args.remove(0)).into()),
            _ => Err(FunctionError::WrongNumberArguments),
        }
    }
    
    fn description() -> &'static str {
       "Natural exponential function - takes only one scalar as arg" 
    }
}
