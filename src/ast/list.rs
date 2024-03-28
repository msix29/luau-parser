//! Implements helper traits for _[lists](List)_

use std::slice::Iter;
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

impl<T> List<T> {
    /// Builds a list from an iterator.
    pub fn from_nodes(
        nodes: Vec<Node>,
        parent_node: Node,
        separators_name: &str,
        code_bytes: &[u8],
        get_item: impl Fn(&Node) -> T,
    ) -> Vec<ListItem<T>> {
        List::from_iter(
            nodes.iter(),
            parent_node,
            separators_name,
            code_bytes,
            get_item,
        )
    }

    /// Builds a list from an iterator.
    pub fn from_iter(
        iterator: Iter<Node>,
        parent_node: Node,
        separators_name: &str,
        code_bytes: &[u8],
        get_item: impl Fn(&Node) -> T,
    ) -> Vec<ListItem<T>> {
        let separators = parent_node
            .children_by_field_name(separators_name, &mut parent_node.walk())
            .collect::<Vec<Node>>();

        iterator
            .enumerate()
            .map(|(i, binding)| {
                if let Some(separator) = separators.get(i) {
                    ListItem::Trailing {
                        item: get_item(binding),
                        separator: SingleToken::from((*separator, code_bytes)),
                    }
                } else {
                    ListItem::NonTrailing(get_item(binding))
                }
            })
            .collect::<Vec<ListItem<T>>>()
    }
}
