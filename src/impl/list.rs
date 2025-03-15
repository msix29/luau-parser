use crate::types::List;

impl<T> List<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }
}
