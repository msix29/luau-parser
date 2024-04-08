//! Implements helper traits for _[lists](List)_

use std::ops::Deref;

use tree_sitter::Node;

use crate::prelude::{List, ListItem, SingleToken};

impl<T> Default for List<T> {
    fn default() -> Self {
        List { items: Vec::new() }
    }
}

impl<T: Clone> List<T> {
    /// Turns `List<T>` into `List<U>` where `U: From<(T, P)>`. `U::from` gets called
    /// with the first paramter being `T` and the second being `P`.
    pub fn to_with_parameters<U: From<(T, P)>, P: Copy>(&self, parameter: P) -> List<U> {
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

    /// Turns `List<T>` into `List<U>` where `U: From<(T, P)>`. `U::from` gets called
    /// with the first paramter being `T` and the second being `P`.
    pub fn to<U: From<T>>(&self) -> List<U> {
        if self.items.is_empty() {
            return List::default();
        }

        List {
            items: self
                .items
                .iter()
                .map(|item| match item {
                    ListItem::Trailing { item, separator } => ListItem::Trailing {
                        item: U::from(item.clone()),
                        separator: separator.clone(),
                    },
                    ListItem::NonTrailing(item) => ListItem::NonTrailing(U::from(item.clone())),
                })
                .collect::<Vec<ListItem<U>>>(),
        }
    }
}
impl<'a, T> List<T> {
    /// Builds a list from an iterator.
    pub fn from_iter<'b>(
        iterator: impl Iterator<Item = Node<'a>> + 'b,
        parent_node: Node,
        separators_name: &str,
        code_bytes: &[u8],
        mut get_item: impl FnMut(usize, Node) -> T,
    ) -> List<T> {
        let separators = parent_node
            .children_by_field_name(separators_name, &mut parent_node.walk())
            .collect::<Vec<Node>>();

        List {
            items: iterator
                .enumerate()
                .map(|(i, binding)| {
                    if let Some(separator) = separators.get(i) {
                        ListItem::Trailing {
                            item: get_item(i, binding),
                            separator: SingleToken::from((*separator, code_bytes)),
                        }
                    } else {
                        ListItem::NonTrailing(get_item(i, binding))
                    }
                })
                .collect::<Vec<ListItem<T>>>(),
        }
    }
}

impl<T> Deref for ListItem<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            ListItem::Trailing { item, separator: _ } => item,
            ListItem::NonTrailing(item) => item,
        }
    }
}
