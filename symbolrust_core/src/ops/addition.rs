use crate::node::Node;
use crate::ops::Negation;

#[derive(Clone, Debug, PartialEq)]
pub struct Addition {
    pub(crate) members: Vec<Node>,
}

impl Addition {
    pub fn new(members: Vec<Node>) -> Self {
        assert!(members.len() > 1);
        Addition { members }
    }

    pub fn from_binary<L: Into<Node>, R: Into<Node>, const INVERSE_OP: bool>(
        lhs: L,
        rhs: R,
    ) -> Self {
        let lhs = lhs.into();
        let rhs = if INVERSE_OP {
            Negation::new(rhs).into()
        } else {
            rhs.into()
        };

        assert!(!matches!(lhs, Node::Addition(_)));
        assert!(!matches!(rhs, Node::Addition(_)));
        Addition {
            members: vec![lhs, rhs],
        }
    }

    pub fn append<N: Into<Node>, const INVERSE_OP: bool>(mut self, rhs: N) -> Self {
        let rhs = if INVERSE_OP {
            Negation::new(rhs).into()
        } else {
            rhs.into()
        };
        assert!(!matches!(rhs, Node::Addition(_)));
        self.members.push(rhs);
        self
    }

    pub fn prepend<N: Into<Node>, const INVERSE_OP: bool>(mut self, lhs: N) -> Self {
        let lhs = lhs.into();
        assert!(!matches!(lhs, Node::Addition(_)));
        let mut n = vec![lhs];
        n.extend(self.members.into_iter().map(|elem| {
            if INVERSE_OP {
                Negation::new(elem).into()
            } else {
                elem
            }
        }));
        self.members = n;
        self
    }

    pub fn fuse<const INVERSE_OP: bool>(mut lhs: Addition, rhs: Addition) -> Self {
        lhs.members.extend(rhs.members.into_iter().map(|elem| {
            if INVERSE_OP {
                Negation::new(elem).into()
            } else {
                elem
            }
        }));
        lhs
    }
}
