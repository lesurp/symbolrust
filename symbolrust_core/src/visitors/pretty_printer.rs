use crate::node::Visitor;
use crate::node::{Node, Precedence};
use crate::ops::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct PrettyPrinter {
    names: HashMap<Variable, String>,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name_var<S: Into<String>>(&mut self, variable: Variable, name: S) {
        self.names.insert(variable, name.into());
    }

    fn build_variadic(&self, m: &[Node], binary_str: &str, precedence: u32) -> String {
        let mut out = String::new();
        for i in 0..m.len() {
            let m_precedence = m[i].precedence().unwrap_or(std::u32::MAX);
            let m_str = m[i].accept_visitor(self);
            let m_str = if m_precedence > precedence {
                m_str
            } else {
                format!("({})", m_str)
            };

            out = format!("{}{}", out, m_str);

            if i < m.len() - 1 {
                out = format!("{} {} ", out, binary_str);
            }
        }
        out
    }
}

impl Visitor for PrettyPrinter {
    type Output = String;

    fn build_exponential(&self, n: &Exponential) -> String {
        let exp = n.exponent.accept_visitor(self);
        "exp(".to_owned() + &exp + ")"
    }

    fn build_power(&self, n: &Power) -> String {
        let xstr = n.val.accept_visitor(self);
        let ystr = n.exponent.accept_visitor(self);
        xstr + "^" + &ystr
    }

    fn build_negation(&self, n: &Negation) -> String {
        "-".to_owned() + &n.val.accept_visitor::<String>(self)
    }

    fn build_inverse(&self, n: &Inverse) -> String {
        "1/".to_owned() + &n.val.accept_visitor::<String>(self)
    }

    fn build_constant(&self, c: &Constant) -> String {
        c.to_string()
    }

    fn build_addition(&self, n: &Addition) -> String {
        let precedence = n.precedence().expect("This should be rewritten anyway...");
        self.build_variadic(&n.members, "+", precedence)
    }

    fn build_multiplication(&self, n: &Multiplication) -> String {
        let precedence = n.precedence().expect("This should be rewritten anyway...");
        self.build_variadic(&n.members, "*", precedence)
    }

    fn build_variable(&self, v: &Variable) -> String {
        if let Some(n) = self.names.get(v) {
            n.clone()
        } else {
            "{undefined}".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrettyPrinter;
    use crate::ops::*;

    #[test]
    fn pretty_printer_add_const() {
        let lhs = Constant::new(123.0354);
        let expr = lhs + -12;

        let pretty_printer = PrettyPrinter::new();
        let out = expr.accept_visitor(&pretty_printer);

        assert_eq!(out, "123.0354 + -12");
    }

    #[test]
    fn pretty_printer_mul_const() {
        let lhs = Constant::new(123.0354);
        let expr = lhs * -12;

        let pretty_printer = PrettyPrinter::new();
        let out = expr.accept_visitor(&pretty_printer);

        assert_eq!(out, "123.0354 * -12");
    }

    #[test]
    fn pretty_printer_add_var() {
        let lhs = Variable::new();
        let expr = lhs + -12;

        let mut pretty_printer = PrettyPrinter::new();
        pretty_printer.name_var(lhs, "mymatrix");
        let out = expr.accept_visitor(&pretty_printer);
        assert_eq!(out, "mymatrix + -12");
    }

    #[test]
    fn pretty_printer_mul_var() {
        let lhs = Variable::new();
        let expr = lhs * -12;

        let mut pretty_printer = PrettyPrinter::new();
        pretty_printer.name_var(lhs, "mymatrix");
        let out = expr.accept_visitor(&pretty_printer);
        assert_eq!(out, "mymatrix * -12");
    }

    #[test]
    fn pretty_printer_add_mul() {
        let x = Variable::new();
        let y = Variable::new();

        let expr = x + 3 * y;

        let mut pretty_printer = PrettyPrinter::new();
        pretty_printer.name_var(x, "x");
        pretty_printer.name_var(y, "y");
        let out = expr.accept_visitor(&pretty_printer);
        assert_eq!(out, "x + 3 * y");
    }

    #[test]
    fn pretty_printer_mul_add() {
        let x = Variable::new();
        let y = Variable::new();

        let expr = (2 + x) * (3 + y);

        let mut pretty_printer = PrettyPrinter::new();
        pretty_printer.name_var(x, "x");
        pretty_printer.name_var(y, "y");
        let out = expr.accept_visitor(&pretty_printer);
        assert_eq!(out, "(2 + x) * (3 + y)");
    }
}
