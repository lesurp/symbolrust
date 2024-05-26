#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Variable(pub(crate) usize);

impl Variable {
    pub fn new(id: usize) -> Self {
        Variable(id)
    }

    pub fn as_raw(&self) -> usize {
        self.0
    }
}
