//! Implements helper traits for [`lists`](List).

use std::ops::Deref;

use tree_sitter::Node;

use crate::{
    prelude::{HasRange, List, ListItem, Range, SingleToken},
    utils::get_range_from_boundaries,
};

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
impl<T> Deref for ListItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Trailing { item, separator: _ } => item,
            Self::NonTrailing(item) => item,
        }
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
    ) -> Self {
        let separators = parent_node
            .children_by_field_name(separators_name, &mut parent_node.walk())
            .collect::<Vec<Node>>();

        Self {
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
impl<T: HasRange> List<T> {
    /// Try getting the range of this list. This function will always return `Some` as
    /// long as there's at least 1 item inside it. If the list is empty it'll be `None`.
    pub fn try_get_range(&self) -> Option<Range> {
        //HACK: If there's only 1 item, no need to call first and last!
        if self.items.len() == 1 {
            Some(self.items[0].get_range())
        } else {
            Some(get_range_from_boundaries(
                self.items.first()?.get_range(),
                self.items.last()?.get_range(),
            ))
        }
    }
}
