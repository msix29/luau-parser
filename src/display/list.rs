//! Implements display traits for lists.

use crate::prelude::{HasRawValue, List, ListItem};

impl<T: HasRawValue> HasRawValue for List<T> {
    fn get_raw_value(&self) -> String {
        self.items
            .iter()
            .map(|item| match item {
                ListItem::Trailing { item, separator } => {
                    format!("{}{}", item.get_raw_value(), separator.get_raw_value())
                }
                ListItem::NonTrailing(item) => item.get_raw_value(),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
