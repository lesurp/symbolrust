use crate::ops::*;

symbolrust_macros::boiler_plate_chef! {
    Addition {
        precedence: 50,
        language_ops: [Add, Sub { inverse: true } ],
    },
    Multiplication {
        precedence: 100,
        language_ops: [Mul, Div { inverse: true } ],
    },
    Power {
        precedence: 150,
        language_ops: [BitXor],
    },
    Variable,
    Constant,
    Negation,
    Log,
}

// TODO: hide being feature flag
impl num_traits::identities::Zero for Node {
    fn zero() -> Self { Node::Constant(Constant::Int(0)) }
    fn is_zero(&self) -> bool { matches!(self, &Node::Constant(Constant::Int(0))) }
}

impl num_traits::identities::One for Node {
    fn one() -> Self { Node::Constant(Constant::Int(1)) }
    fn is_one(&self) -> bool { matches!(self, &Node::Constant(Constant::Int(1))) }
}

impl ndarray::ScalarOperand for Node {}
