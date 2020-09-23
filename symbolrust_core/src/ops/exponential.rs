use crate::function::Function;
use crate::node::Node;

/// TODO: make some generic Binary struct so we can more easily implement visitors
#[derive(Clone, Debug, PartialEq)]
pub struct Exponential {
    pub(crate) exponent: Box<Node>,
}

impl Exponential {
    pub fn new(exponent: Node) -> Self {
        Exponential {
            exponent: exponent.into(),
        }
    }
}

impl Function for Exponential {
    fn from_args(mut args: Vec<Node>) -> Result<Node, ()> {
        match args.len() {
            1 => Ok(Exponential::new(args.remove(0)).into()),
            _ => Err(()),
        }
    }
}
