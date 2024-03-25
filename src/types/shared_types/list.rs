use super::SingleToken;

#[derive(Clone, Debug)]
pub enum ListItem<T> {
    Trailing {
        item: T,
        separator: SingleToken,
    },
    NonTrailing(T)
}

#[derive(Clone, Debug)]
pub struct List<T> {
    pub items: Vec<ListItem<T>>
}
