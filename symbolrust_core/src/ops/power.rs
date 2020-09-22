use crate::node::Node;

/// TODO: make some generic Binary struct so we can more easily implement visitors
#[derive(Clone, Debug, PartialEq)]
pub struct Power {
    pub(crate) val: Box<Node>,
    pub(crate) exponent: Box<Node>,
}

impl Power {
    pub fn new(val: Node, exponent: Node) -> Self {
        Power {
            val: val.into(),
            exponent: exponent.into(),
        }
    }

    pub fn from_binary<L: Into<Node>, R: Into<Node>, const INVERSE_OP: bool>(
        lhs: L,
        rhs: R,
    ) -> Self {
        Power {
            val: lhs.into().into(),
            exponent: rhs.into().into(),
        }
    }

    pub fn append<N: Into<Node>, const INVERSE_OP: bool>(self, rhs: N) -> Self {
        Power {
            val: self.into(),
            exponent: rhs.into().into(),
        }
    }

    pub fn prepend<N: Into<Node>, const INVERSE_OP: bool>(self, lhs: N) -> Self {
        Power {
            val: lhs.into().into(),
            exponent: self.into(),
        }
    }

    pub fn fuse<const INVERSE_OP: bool>(lhs: Power, rhs: Power) -> Self {
        Power {
            val: lhs.into(),
            exponent: rhs.into(),
        }
    }
}
