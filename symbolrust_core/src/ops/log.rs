use crate::function::Function;
use crate::node::Node;

/// TODO: make some generic Binary struct so we can more easily implement visitors
#[derive(Clone, Debug, PartialEq)]
pub struct Log {
    pub(crate) val: Box<Node>,
}

impl Log {
    pub fn new(val: Node) -> Self {
        Log {
            val: val.into(),
        }
    }
}

impl Function for Log {
    fn from_args(mut args: Vec<Node>) -> Result<Node, ()> {
        match args.len() {
            1 => Ok(Log::new(args.remove(0)).into()),
            _ => Err(()),
        }
    }
}

