use crate::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Negation {
    pub(crate) val: Box<Node>,
}

impl Negation {
    pub fn new<N: Into<Node>>(val: N) -> Self {
        Negation {
            val: val.into().into(),
        }
    }
}
