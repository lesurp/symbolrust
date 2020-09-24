use crate::node::Node;
use crate::ops::Power;

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

    pub fn from_binary<L: Into<Node>, R: Into<Node>, const INVERSE_OP: bool>(
        lhs: L,
        rhs: R,
    ) -> Self {
        match (lhs.into(), rhs.into()) {
            (Node::Multiplication(lhs), Node::Multiplication(rhs)) => {
                Multiplication::fuse::<INVERSE_OP>(lhs, rhs)
            }
            (Node::Multiplication(lhs), rhs) => lhs.append::<_, INVERSE_OP>(rhs),
            (lhs, Node::Multiplication(rhs)) => rhs.prepend::<_, INVERSE_OP>(lhs),
            (lhs, rhs) => {
                let rhs = if INVERSE_OP {
                    Power::inverse(rhs).into()
                } else {
                    rhs
                };
                Multiplication {
                    members: vec![lhs, rhs],
                }
            }
        }
    }

    pub fn append<N: Into<Node>, const INVERSE_OP: bool>(mut self, rhs: N) -> Self {
        let rhs = if INVERSE_OP {
            Power::inverse(rhs).into()
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
                Power::inverse(elem).into()
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
                Power::inverse(elem).into()
            } else {
                elem
            }
        }));
        lhs
    }
}
