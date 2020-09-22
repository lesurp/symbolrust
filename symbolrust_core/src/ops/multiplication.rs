use crate::node::Node;
use crate::ops::Inverse;

/// TODO: make some generic Binary struct so we can more easily implement visitors
#[derive(Clone, Debug, PartialEq)]
pub struct Multiplication {
    pub(crate) members: Vec<Node>,
}

impl Multiplication {
    pub fn new(members: Vec<Node>) -> Self {
        assert!(members.len() > 1);
        Multiplication { members }
    }

    pub fn from_binary<L: Into<Node>, R: Into<Node>, const INVERSE_OP: bool>( lhs: L,
        rhs: R,
    ) -> Self {
        let lhs = lhs.into();
        let rhs = if INVERSE_OP {
            Inverse::new(rhs).into()
        } else {
            rhs.into()
        };
        assert!(!matches!(lhs, Node::Multiplication(_)));
        assert!(!matches!(rhs, Node::Multiplication(_)));
        Multiplication {
            members: vec![lhs, rhs],
        }
    }

    pub fn append<N: Into<Node>, const INVERSE_OP: bool>(mut self, rhs: N) -> Self {
        let rhs = if INVERSE_OP {
            Inverse::new(rhs).into()
        } else {
            rhs.into()
        };
        assert!(!matches!(rhs, Node::Multiplication(_)));
        self.members.push(rhs);
        self
    }

    pub fn prepend<N: Into<Node>, const INVERSE_OP: bool>(mut self, lhs: N) -> Self {
        let lhs = lhs.into();
        assert!(!matches!(lhs, Node::Multiplication(_)));
        let mut n = vec![lhs];
        n.extend(self.members.into_iter().map(|elem| {
            if INVERSE_OP {
                Inverse::new(elem).into()
            } else {
                elem
            }
        }));
        self.members = n;
        self
    }

    pub fn fuse<const INVERSE_OP: bool>(mut lhs: Multiplication, rhs: Multiplication) -> Self {
        lhs.members.extend(rhs.members.into_iter().map(|elem| {
            if INVERSE_OP {
                Inverse::new(elem).into()
            } else {
                elem
            }
        }));
        lhs
    }
}
