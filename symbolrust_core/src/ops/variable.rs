use crate::node::Node;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type VariableContext = HashMap<Variable, Node>;

static GLOBAL_CONTEXT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Variable {
    pub(crate) id: usize,
}

impl Variable {
    pub fn new() -> Self {
        let id = GLOBAL_CONTEXT.fetch_add(1, Ordering::Relaxed);
        Variable { id }
    }
}
