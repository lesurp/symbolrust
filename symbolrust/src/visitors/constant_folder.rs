use crate::node::{Node, Visitor};
use crate::ops::*;

pub struct ConstantFolder;
impl ConstantFolder {
    pub fn fold_f64(n: &Node) -> Option<f64> {
        let n = n.accept_visitor(&ConstantFolder);
        match n {
            Node::Constant(Constant::Int(i)) => Some(i as f64),
            Node::Constant(Constant::Fp(f)) => Some(f),
            _ => None,
        }
    }

    pub fn fold_i64(n: &Node) -> Option<i64> {
        let n = n.accept_visitor(&ConstantFolder);
        match n {
            Node::Constant(Constant::Int(i)) => Some(i),
            Node::Constant(Constant::Fp(f)) => Some(f as i64),
            _ => None,
        }
    }

    pub fn fold(n: &Node) -> Node {
        n.accept_visitor(&ConstantFolder)
    }
}

impl Visitor for ConstantFolder {
    type Output = Node;

    fn build_log(&self, n: &Log) -> Node {
        let val = n.val.accept_visitor(self);
        match val {
            Node::Constant(Constant::Int(exp)) => (exp as f64).log2().into(),
            Node::Constant(Constant::Fp(exp)) => exp.log2().into(),
            _ => Log::new(val).into(),
        }
    }

    fn build_power(&self, n: &Power) -> Node {
        let val = n.val.accept_visitor(self);
        let exponent = n.exponent.accept_visitor(self);
        match (val, exponent) {
            (Node::Constant(Constant::Int(x)), Node::Constant(Constant::Int(y))) => {
                // FIXME: error handling?
                let res = x.checked_pow(y.abs() as u32).unwrap();
                if y < 0 {
                    (1.0 / res as f64).into()
                } else {
                    res.into()
                }
            }
            (Node::Constant(Constant::Fp(x)), Node::Constant(Constant::Int(y))) => {
                Constant::new(x.powi(y as i32)).into()
            }
            (Node::Constant(Constant::Fp(x)), Node::Constant(Constant::Fp(y))) => {
                Constant::new(x.powf(y)).into()
            }
            (Node::Constant(Constant::Int(x)), Node::Constant(Constant::Fp(y))) => {
                Constant::new((x as f64).powf(y)).into()
            }
            (x, Node::Constant(Constant::Int(y))) => match y {
                0 => Constant::new(1).into(),
                1 => x,
                _ => n.clone().into(),
            },
            _ => n.clone().into(),
        }
    }

    fn build_negation(&self, n: &Negation) -> Node {
        match n.val.as_ref() {
            Node::Constant(Constant::Int(i)) => Constant::new(-i).into(),
            Node::Constant(Constant::Fp(f)) => Constant::new(-f).into(),
            Node::Negation(nested_n) => nested_n.clone().into(),
            _ => n.clone().into(),
        }
    }

    fn build_addition(&self, n: &Addition) -> Node {
        let mut acc_f = None;
        let mut acc_i = None;
        let mut acc_n = Vec::new();

        for m in &n.members {
            let m = m.accept_visitor(self);
            match m {
                Node::Constant(Constant::Int(i)) => acc_i = Some(acc_i.unwrap_or(0) + i),
                Node::Constant(Constant::Fp(f)) => acc_f = Some(acc_f.unwrap_or(0.0) + f),
                n => acc_n.push(n.clone()),
            }
        }

        match (acc_i, acc_f) {
            (Some(i), None) => {
                if i != 0 {
                    acc_n.push(i.into());
                }
            }
            (None, Some(f)) => {
                acc_n.push(f.into());
            }
            (None, None) => {}
            (Some(i), Some(f)) => {
                acc_n.push((i as f64 + f).into());
            }
        }

        // if everything folds to 0, we can get 0 elements here
        match acc_n.len() {
            0 => 0.into(),
            1 => acc_n.pop().unwrap(),
            _ => Addition::new(acc_n).into(),
        }
    }

    fn build_multiplication(&self, n: &Multiplication) -> Node {
        let mut acc_f = None;
        let mut acc_i = None;
        let mut acc_n = Vec::new();

        for m in &n.members {
            let m = m.accept_visitor(self);
            match m {
                // short-circuit if we multiply by 0
                Node::Constant(Constant::Int(0)) => return 0.into(),
                Node::Constant(Constant::Int(i)) => acc_i = Some(acc_i.unwrap_or(1) * i),
                Node::Constant(Constant::Fp(f)) => acc_f = Some(acc_f.unwrap_or(1.0) * f),
                n => acc_n.push(n.clone()),
            }
        }

        match (acc_i, acc_f) {
            (Some(i), None) => {
                // remove op with identity
                if i != 1 {
                    acc_n.push(i.into());
                }
            }
            (None, Some(f)) => {
                acc_n.push(f.into());
            }
            (None, None) => {}
            (Some(i), Some(f)) => {
                acc_n.push((i as f64 * f).into());
            }
        }

        match acc_n.len() {
            1 => acc_n.pop().unwrap(),
            _ => Multiplication::new(acc_n).into(),
        }
    }

    fn build_constant(&self, c: &Constant) -> Node {
        (*c).into()
    }

    fn build_variable(&self, v: &Variable) -> Node {
        (*v).into()
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantFolder;
    use crate::node::Node;
    use crate::ops::*;

    #[test]
    fn constant_folder_fi_add() {
        let lhs = Constant::new(123.0354);
        let rhs = -12;
        let expr_fi = lhs + rhs;
        let expr_if = rhs + lhs;

        let folded_fi = expr_fi.accept_visitor(&ConstantFolder);
        let folded_if = expr_if.accept_visitor(&ConstantFolder);
        let expected = Constant::new(123.0354 - 12_f64);
        assert_eq!(folded_fi, Node::Constant(expected));
        assert_eq!(folded_fi, folded_if);
    }

    #[test]
    fn constant_folder_fi_sub() {
        let lhs = Constant::new(123.0354);
        let rhs = 12;
        let expr_fi = lhs - rhs;
        let expr_if = -rhs + lhs;

        let folded_fi = expr_fi.accept_visitor(&ConstantFolder);
        let folded_if = expr_if.accept_visitor(&ConstantFolder);
        let expected = Constant::new(123.0354 - 12_f64);
        assert_eq!(folded_fi, Node::Constant(expected));
        assert_eq!(folded_fi, folded_if);
    }

    #[test]
    fn constant_folder_fi_mul() {
        let lhs = Constant::new(123.0354);
        let rhs = -12;
        let expr_fi = lhs * rhs;
        let expr_if = rhs * lhs;

        let folded_fi = expr_fi.accept_visitor(&ConstantFolder);
        let folded_if = expr_if.accept_visitor(&ConstantFolder);
        let expected = Constant::new(123.0354 * -12.0);
        assert_eq!(folded_fi, Node::Constant(expected));
        assert_eq!(folded_fi, folded_if);
    }

    #[test]
    fn constant_folder_fi_div() {
        let lhs = Constant::new(123.0354);
        let rhs = -12;

        {
            let expr_fi = lhs / rhs;
            let folded_fi = ConstantFolder::fold_f64(&expr_fi).unwrap();
            let expected_fi = 123.0354 / -12.0;
            assert_eps!(folded_fi, expected_fi);
        }

        {
            let expr_if = rhs / lhs;
            let folded_if = ConstantFolder::fold_f64(&expr_if).unwrap();
            let expected_if = -12.0 / 123.0354;
            assert_eps!(folded_if, expected_if);
        }
    }

    #[test]
    fn constant_folder_ii_add() {
        let lhs = Constant::new(234);
        let rhs = -12;
        let expr = lhs + rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        let expected = Constant::new(222);
        assert_eq!(folded, Node::Constant(expected));
    }

    #[test]
    fn constant_folder_ii_sub() {
        let lhs = Constant::new(234);
        let rhs = 12;
        let expr = lhs - rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        let expected = Constant::new(222);
        assert_eq!(folded, Node::Constant(expected));
    }

    #[test]
    fn constant_folder_ii_mul() {
        let lhs = Constant::new(234);
        let rhs = -12;
        let expr = lhs * rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        let expected = Constant::new(-12 * 234);
        assert_eq!(folded, Node::Constant(expected));
    }

    #[test]
    fn constant_folder_ii_div() {
        let lhs = Constant::new(234);
        let rhs = -12;
        let expr = lhs / rhs;

        let folded = ConstantFolder::fold_f64(&expr).unwrap();
        let expected = 234.0 / -12.0;
        assert_eps!(folded, expected);
    }

    #[test]
    fn constant_folder_with_variable_add() {
        let lhs = Variable::new();
        let rhs = -12;
        let expr = lhs + rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        assert_eq!(folded, expr);
    }

    #[test]
    fn constant_folder_with_variable_sub() {
        let lhs = Variable::new();
        let rhs = -12;
        let expr = lhs - rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        let expected = lhs + 12;
        assert_eq!(folded, expected);
    }

    #[test]
    fn constant_folder_with_variable_mul() {
        let lhs = Variable::new();
        let rhs = -12;
        let expr = lhs * rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        assert_eq!(folded, expr);
    }

    #[test]
    fn constant_folder_with_variable_div() {
        let lhs = Variable::new();
        let rhs = -12;
        let expr = lhs / rhs;

        let folded = expr.accept_visitor(&ConstantFolder);
        let expected = lhs * (-1.0 / 12.0);
        assert_eq!(folded, expected);
    }

    #[test]
    fn constant_folder_add_mul() {
        let x = Constant::new(1);
        let expr = (x + 2) * 10;

        let folded = expr.accept_visitor(&ConstantFolder);
        assert_eq!(folded, Node::Constant(Constant::new(30)));
    }

    #[test]
    fn constant_folder_sub_mul() {
        let x = Constant::new(1);
        let expr = (x - 2) * 10;

        let folded = expr.accept_visitor(&ConstantFolder);
        assert_eq!(folded, Node::Constant(Constant::new(-10)));
    }

    #[test]
    fn constant_folder_add_div() {
        let x = Constant::new(1);
        let expr = (x + 2) / 10;

        let folded = ConstantFolder::fold_f64(&expr).unwrap();
        assert_eps!(folded, 0.3);
    }

    #[test]
    fn constant_folder_sub_div() {
        let x = Constant::new(1);
        let expr = (x - 2) / 10;

        let folded = expr.accept_visitor(&ConstantFolder);
        assert_eq!(folded, Node::Constant(Constant::new(-0.1)));
    }
}
