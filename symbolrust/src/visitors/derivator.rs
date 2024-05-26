use crate::node::{Node, Visitor};
use crate::ops::*;

pub struct Derivator {
    x: Variable,
}

impl Visitor for Derivator {
    type Output = Node;

    fn build_log(&self, n: &Log) -> Node {
        let val = n.val.accept_visitor(self);
        val / n.clone()
    }

    fn build_power(&self, n: &Power) -> Node {
        let x_dot = n.val.accept_visitor(self);
        let y_dot = n.exponent.accept_visitor(self);
        let scalar =
            y_dot * Log::new(n.val.as_ref().clone()) + x_dot * n.exponent.as_ref() / n.val.as_ref();
        n.clone() * scalar
    }

    fn build_negation(&self, n: &Negation) -> Node {
        let val = n.val.accept_visitor(self);
        Negation::new(val).into()
    }

    fn build_addition(&self, n: &Addition) -> Node {
        let ms = n.members.iter().map(|m| m.accept_visitor(self)).collect();
        Addition::new(ms).into()
    }

    fn build_constant(&self, _c: &Constant) -> Node {
        0.into()
    }

    fn build_variable(&self, v: &Variable) -> Node {
        if *v == self.x { 1 } else { 0 }.into()
    }

    fn build_multiplication(&self, n: &Multiplication) -> Node {
        let all_der = n
            .members
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let dm = m.accept_visitor(self);
                let mut elem = n.members.clone();
                elem[i] = dm;
                Multiplication::new(elem).into()
            })
            .collect();

        Addition::new(all_der).into()
    }
}

impl Derivator {
    pub fn new(x: Variable) -> Self {
        Derivator { x }
    }
}

#[cfg(test)]
mod tests {
    use super::Derivator;
    use crate::context::Context;
    use crate::ops::*;
    use crate::visitors::{ConstantFolder, Evaluator};

    #[test]
    fn derive_complete() {
        let mut variables = Context::new();
        vars! {variables,
            let x;
            let y;
        }
        let rhs = -12 * y;

        // g(x, y) = x - 12y
        let expr = x + rhs;

        let fx = x + 3;
        variables.assign(y, fx);

        // h(x) = g(x, f(x)) = x - 12 * (3 + x) = -(11x + 36)
        // dh/dx = -11
        let evaluator = Evaluator::new(&variables);
        let derivator = Derivator::new(x);
        let expr = expr.accept_visitor(&evaluator);
        let expr = expr.accept_visitor(&derivator);
        let expr = expr.accept_visitor(&ConstantFolder);

        assert_eq!(expr, (-11).into());
    }

    #[test]
    fn derive_incomplete() {
        let x = Variable::new();
        let y = Variable::new();

        // g(x, y) = x - 12y
        let expr = x + y * (-12);

        // dg/dx = 1
        let derivator = Derivator::new(x);
        let expr = expr.accept_visitor(&derivator);
        let expr = expr.accept_visitor(&ConstantFolder);

        assert_eq!(expr, 1.into());
    }
}
