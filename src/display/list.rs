//! Implements display traits for lists.

use crate::prelude::{HasRawValue, List, ListItem, Print};

/// Implement a `trait` for [`List<T>`] where `T: trait`.
macro_rules! impl_list {
    ($trait: ident, $fn_name: ident) => {
        impl<T: $trait> $trait for List<T> {
            fn $fn_name(&self) -> String {
                self.items
                    .iter()
                    .map(|item| match item {
                        ListItem::Trailing { item, separator } => {
                            format!("{}{}", item.$fn_name(), separator.$fn_name())
                        }
                        ListItem::NonTrailing(item) => item.$fn_name(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            }
        }
    };
}

impl_list!(HasRawValue, get_raw_value);
impl_list!(Print, print);
