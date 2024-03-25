use crate::prelude::{List, ListItem};

impl<T> Default for List<T> {
    fn default() -> Self {
        List { items: Vec::new() }
    }
}

impl<T: Clone> List<T> {
    pub fn to<U: From<(T, P)>, P: Copy>(&self, parameter: P) -> List<U> {
        if self.items.is_empty() {
            return List::default();
        }

        List {
            items: self
                .items
                .iter()
                .map(|item| match item {
                    ListItem::Trailing { item, separator } => ListItem::Trailing {
                        item: U::from((item.clone(), parameter)),
                        separator: separator.clone(),
                    },
                    ListItem::NonTrailing(item) => {
                        ListItem::NonTrailing(U::from((item.clone(), parameter)))
                    }
                })
                .collect::<Vec<ListItem<U>>>(),
        }
    }
}
