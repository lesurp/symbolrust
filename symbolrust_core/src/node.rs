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
