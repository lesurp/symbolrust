use crate::node::Node;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Constant {
    Fp(f64),
    Int(i64),
}

impl Constant {
    pub fn new<C: Into<Constant>>(c: C) -> Self {
        c.into()
    }

    pub fn as_float(self) -> f64 {
        match self {
            Constant::Fp(value) => value,
            Constant::Int(value) => value as f64,
        }
    }
}

impl Into<Constant> for i64 {
    fn into(self) -> Constant {
        Constant::Int(self)
    }
}

impl Into<Constant> for f64 {
    fn into(self) -> Constant {
        Constant::Fp(self)
    }
}

impl Into<Node> for i64 {
    fn into(self) -> Node {
        Node::Constant(Constant::Int(self))
    }
}

impl Into<Node> for f64 {
    fn into(self) -> Node {
        Node::Constant(Constant::Fp(self))
    }
}

impl ToString for Constant {
    fn to_string(&self) -> String {
        match self {
            Constant::Fp(f) => f.to_string(),
            Constant::Int(i) => i.to_string(),
        }
    }
}
