use std::collections::{hash_map::Entry, HashMap};

use crate::{node::Node, ops::Variable, visitors::PrettyPrinterContext};

#[derive(Default)]
pub struct Context {
    v2n: PrettyPrinterContext,
    n2v: HashMap<String, Variable>,
    values: HashMap<Variable, Node>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_var<S: Into<String>>(&mut self, n: S) -> Variable {
        match self.n2v.entry(n.into()) {
            Entry::Vacant(vac) => {
                let var = Variable::new();
                self.v2n.name_var(var, vac.key().clone());
                *vac.insert(var)
            }
            Entry::Occupied(occ) => *occ.get(),
        }
    }

    pub fn assign(&mut self, v: Variable, n: Node) {
        assert!(self.v2n.as_map().contains_key(&v));
        self.values.insert(v, n);
    }

    pub fn get_name(&mut self, v: Variable) -> Option<&String> {
        self.v2n.as_map().get(&v)
    }

    pub fn as_printer(&self) -> &PrettyPrinterContext {
        &self.v2n
    }

    pub fn value(&self, v: &Variable) -> Option<&Node> {
        self.values.get(v)
    }

    pub fn values(&self) -> &HashMap<Variable, Node> {
        &self.values
    }

    pub fn print(&self, n: &Node) -> String {
        self.v2n.print(n)
    }
}
