use crate::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Inverse {
    pub(crate) val: Box<Node>,
}

impl Inverse {
    pub fn new<N: Into<Node>>(val: N) -> Self {
        Inverse {
            val: val.into().into(),
        }
    }
}

