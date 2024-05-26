use std::sync::atomic::{AtomicUsize, Ordering};

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

    pub fn as_raw(&self) -> usize {
        self.id
    }
}
