use crate::node::{Node, Visitor};
use crate::ops::*;

pub struct Evaluator<'a> {
    variables: &'a VariableContext,
}

impl<'a> Visitor for Evaluator<'a> {
    type Output = Node;

    fn build_log(&self, n: &Log) -> Node {
        let val = n.val.accept_visitor(self);
        Log::new(val).into()
    }

    fn build_power(&self, n: &Power) -> Node {
        let x = n.val.accept_visitor(self);
        let y = n.exponent.accept_visitor(self);
        x ^ y
    }

    fn build_negation(&self, n: &Negation) -> Node {
        -n.val.accept_visitor(self)
    }

    fn build_addition(&self, n: &Addition) -> Node {
        let ms = n.members.iter().map(|m| m.accept_visitor(self)).collect();
        Addition::new(ms).into()
    }

    fn build_constant(&self, c: &Constant) -> Node {
        (*c).into()
    }

    fn build_variable(&self, v: &Variable) -> Node {
        if let Some(def) = self.variables.get(v) {
            // TODO: this clone would not be needed is the trait did not take mut
            // is it reasonable to pay such price for visitors that do NOT mutate state?
            // should we duplicate traits so visitors can choose to be const or not?
            def.clone().accept_visitor(self)
        } else {
            (*v).into()
        }
    }

    fn build_multiplication(&self, n: &Multiplication) -> Node {
        let ms = n.members.iter().map(|m| m.accept_visitor(self)).collect();
        Multiplication::new(ms).into()
    }
}

impl<'a> Evaluator<'a> {
    pub fn new(variables: &'a VariableContext) -> Self {
        Evaluator { variables }
    }

    pub fn evaluate(n: &Node, variables: &'a VariableContext) -> Node {
        n.accept_visitor(&Evaluator::new(variables))
    }
}

#[cfg(test)]
mod tests {
    use super::Evaluator;
    use crate::ops::*;
    use crate::visitors::ConstantFolder;

    #[test]
    fn evaluate_complete() {
        let x = Variable::new();
        let y = Variable::new();
        let rhs = -12 * y;

        let mut variables = VariableContext::new();
        let fx = x + 3;
        variables.insert(y, fx);

        variables.insert(x, 23.into());

        // g(x, y) = x - 12y
        let expr = x + rhs;

        let evaluator = Evaluator::new(&variables);
        let evaluate = expr.accept_visitor(&evaluator);
        let evaluate_folded = evaluate.accept_visitor(&ConstantFolder);

        // h(x) = g(x, f(x)) = x - 12 * (3 + x) = -(11x + 36)
        // h(23) = -289
        assert_eq!(evaluate_folded, (-289).into());
    }

    #[test]
    fn evaluate_incomplete() {
        let x = Variable::new();
        let y = Variable::new();
        let rhs = -12 * y;

        // g(x, y) = x - 12y
        let expr = x + rhs;
        assert_eq!(expr, x + -12 * y);

        let mut variables = VariableContext::new();
        variables.insert(x, 23.into());

        let evaluator = Evaluator::new(&variables);
        let evaluate = expr.accept_visitor(&evaluator);

        let expected = 23 + -12 * y;
        assert_eq!(evaluate, expected);
    }
}
