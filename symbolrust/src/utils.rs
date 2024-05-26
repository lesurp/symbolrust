use std::collections::{hash_map::Entry, HashMap};

use crate::{node::Node, ops::Variable, visitors::PrettyPrinterContext};

#[allow(unused_macros)]
macro_rules! assert_eps {
    ($left:expr, $right:expr, $eps:expr) => {{
        let left = $left;
        let right = $right;
        let eps = $eps;
        let delta = (left - right).abs();
        if delta > eps {
            // The reborrows below are intentional. Without them, the stack slot for the
            // borrow is initialized even before the values are compared, leading to a
            // noticeable slow down.
            panic!(
                r#"assertion failed: `(left ~= right (esp))`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`"#,
                left, right, eps
            );
        }
    }};
    ($left:expr, $right:expr) => {
        assert_eps!($left, $right, 1e-4);
    };
}

pub type VariableContext = HashMap<Variable, Node>;

#[derive(Default)]
pub struct VariableMap {
    v2n: PrettyPrinterContext,
    n2v: HashMap<String, Variable>,
}

impl VariableMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name<S: Into<String>>(&mut self, n: S) -> Variable {
        match self.n2v.entry(n.into()) {
            Entry::Vacant(vac) => {
                let var = Variable::new();
                self.v2n.name_var(var, vac.key().clone());
                *vac.insert(var)
            }
            Entry::Occupied(occ) => *occ.get(),
        }
    }

    pub fn var(&mut self, v: Variable) -> Option<&String> {
        self.v2n.as_map().get(&v)
    }

    pub fn as_context(&self) -> &PrettyPrinterContext {
        &self.v2n
    }
}
