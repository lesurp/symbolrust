use crate::node::Node;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Constant {
    Fp(f64),
    Int(i64),
    E,
    Pi,
}

impl Constant {
    pub fn new<C: Into<Constant>>(c: C) -> Self {
        c.into()
    }

    pub fn as_float(self) -> f64 {
        match self {
            Constant::Fp(value) => value,
            Constant::Int(value) => value as f64,
            Constant::E => std::f64::consts::E,
            Constant::Pi => std::f64::consts::PI,
        }
    }
}

impl From<i64> for Constant {
    fn from(val: i64) -> Constant {
        Constant::Int(val)
    }
}

impl From<f64> for Constant {
    fn from(val: f64) -> Constant {
        Constant::Fp(val)
    }
}

impl From<i64> for Node {
    fn from(val: i64) -> Node {
        Node::Constant(Constant::Int(val))
    }
}

impl From<f64> for Node {
    fn from(val: f64) -> Node {
        Node::Constant(Constant::Fp(val))
    }
}

impl ToString for Constant {
    fn to_string(&self) -> String {
        match self {
            Constant::Int(i) => i.to_string(),
            f => f.as_float().to_string(),
        }
    }
}
