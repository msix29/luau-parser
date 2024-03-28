//! Implements display traits for lists.

use std::fmt::Display;

use crate::prelude::{List, ListItem};

impl<T: Display> List<T> {
    /// Joines the list as one string.
    pub fn join(&self) -> String {
        self.items
            .iter()
            .map(|item| match item {
                ListItem::Trailing { item, separator } => format!("{}{}", item, separator),
                ListItem::NonTrailing(item) => item.to_string(),
            })
            .collect::<String>()
    }
}
