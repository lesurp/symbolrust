use crate::node::Visitor;
use crate::node::{Node, Precedence};
use crate::ops::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct PrettyPrinterContext(HashMap<Variable, String>);
impl PrettyPrinterContext {
    pub fn new() -> Self {
        PrettyPrinterContext::default()
    }

    pub fn name_var<S: Into<String>>(&mut self, variable: Variable, name: S) {
        self.0.insert(variable, name.into());
    }

    pub fn as_map(&self) -> &HashMap<Variable, String> {
        &self.0
    }

    pub fn as_mut_map(&mut self) -> &mut HashMap<Variable, String> {
        &mut self.0
    }

    pub fn print(&self, n: &Node) -> String {
        PrettyPrinter::print_with_context(n, self)
    }
}

pub struct PrettyPrinter<'a> {
    context: &'a PrettyPrinterContext,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(context: &'a PrettyPrinterContext) -> Self {
        PrettyPrinter { context }
    }

    pub fn print(n: &Node) -> String {
        let c = PrettyPrinterContext::default();
        let pp = PrettyPrinter::new(&c);
        n.accept_visitor(&pp)
    }

    pub fn print_with_context(n: &Node, context: &PrettyPrinterContext) -> String {
        let pp = PrettyPrinter::new(context);
        n.accept_visitor(&pp)
    }

    // TODO: ndarray feature flag
    pub fn print_array<S: ndarray::Data + ndarray::RawData<Elem = Node>>(
        arr: &ndarray::ArrayBase<S, ndarray::Ix2>,
    ) -> String {
        let c = PrettyPrinterContext::default();
        PrettyPrinter::print_array_with_context(arr, &c)
    }

    // TODO: ndarray feature flag
    // TODO: can I be generic over array size ? Me think not (mb with some macro hell...)
    pub fn print_array_with_context<S: ndarray::Data + ndarray::RawData<Elem = Node>>(
        arr: &ndarray::ArrayBase<S, ndarray::Ix2>,
        context: &PrettyPrinterContext,
    ) -> String {
        let mut s = String::from("[");
        arr.indexed_iter().for_each(|((i, j), var)| {
            if j == 0 {
                if i != 0 {
                    s += " ";
                }
                s += "[";
            }
            s += &PrettyPrinter::print_with_context(var, context);
            s += match (i + 1 == arr.nrows(), j + 1 == arr.ncols()) {
                (false, true) => "],\n",
                (true, true) => "]]\n",
                (_, false) => ",\t",
            };
        });
        s
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::print(self))
    }
}

fn build_variadic<P: Visitor<Output = String>>(
    pretty_printer: &P,
    m: &[Node],
    binary_str: &str,
    precedence: u32,
) -> String {
    let mut out = String::new();
    for i in 0..m.len() {
        let m_precedence = m[i].precedence().unwrap_or(u32::MAX);
        let m_str = m[i].accept_visitor(pretty_printer);
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

impl<'a> Visitor for PrettyPrinter<'a> {
    type Output = String;

    fn build_log(&self, n: &Log) -> String {
        let val = n.val.accept_visitor(self);
        "ln(".to_owned() + &val + ")"
    }

    fn build_power(&self, n: &Power) -> String {
        let ystr = n.exponent.accept_visitor(self);
        match (n.val.as_ref(), n.exponent.as_ref()) {
            (Node::Constant(Constant::E), _) => "exp(".to_owned() + &ystr + ")",
            _ => {
                let xstr = n.val.accept_visitor(self);
                xstr + "^" + &ystr
            }
        }
    }

    fn build_negation(&self, n: &Negation) -> String {
        "-".to_owned() + &n.val.accept_visitor::<String>(self)
    }

    fn build_constant(&self, c: &Constant) -> String {
        c.to_string()
    }

    fn build_addition(&self, n: &Addition) -> String {
        let precedence = n.precedence().expect("This should be rewritten anyway...");
        build_variadic(self, &n.members, "+", precedence)
    }

    fn build_multiplication(&self, n: &Multiplication) -> String {
        let precedence = n.precedence().expect("This should be rewritten anyway...");
        build_variadic(self, &n.members, "*", precedence)
    }

    fn build_variable(&self, v: &Variable) -> String {
        if let Some(n) = self.context.0.get(v) {
            n.clone()
        } else {
            "{undefined}".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PrettyPrinter, PrettyPrinterContext};
    use crate::ops::*;

    #[test]
    fn pretty_printer_add_const() {
        let lhs = Constant::new(123.0354);
        let expr = lhs + -12;

        let out = PrettyPrinter::print(&expr);

        assert_eq!(out, "123.0354 + -12");
    }

    #[test]
    fn pretty_printer_mul_const() {
        let lhs = Constant::new(123.0354);
        let expr = lhs * -12;

        let out = PrettyPrinter::print(&expr);

        assert_eq!(out, "123.0354 * -12");
    }

    #[test]
    fn pretty_printer_add_var() {
        let lhs = Variable::new();
        let expr = lhs + -12;

        let mut pp_context = PrettyPrinterContext::new();
        pp_context.name_var(lhs, "mymatrix");
        let out = PrettyPrinter::print_with_context(&expr, &pp_context);
        assert_eq!(out, "mymatrix + -12");
    }

    #[test]
    fn pretty_printer_mul_var() {
        let lhs = Variable::new();
        let expr = lhs * -12;

        let mut pp_context = PrettyPrinterContext::new();
        pp_context.name_var(lhs, "mymatrix");
        let out = PrettyPrinter::print_with_context(&expr, &pp_context);
        assert_eq!(out, "mymatrix * -12");
    }

    #[test]
    fn pretty_printer_add_mul() {
        let x = Variable::new();
        let y = Variable::new();

        let expr = x + 3 * y;

        let mut pp_context = PrettyPrinterContext::new();
        pp_context.name_var(x, "x");
        pp_context.name_var(y, "y");
        let out = PrettyPrinter::print_with_context(&expr, &pp_context);
        assert_eq!(out, "x + 3 * y");
    }

    #[test]
    fn pretty_printer_mul_add() {
        let x = Variable::new();
        let y = Variable::new();

        let expr = (2 + x) * (3 + y);

        let mut pp_context = PrettyPrinterContext::new();
        pp_context.name_var(x, "x");
        pp_context.name_var(y, "y");
        let out = PrettyPrinter::print_with_context(&expr, &pp_context);
        assert_eq!(out, "(2 + x) * (3 + y)");
    }
}
