//! Implements display traits for lists.
use crate::prelude::{HasRawValue, List, ListItem, Print};

/// Implement a `trait` for [`List<T>`] where `T: trait`.
macro_rules! impl_list {
    ($trait: ident, $fn_name: ident, $join_with: literal) => {
        impl<T: $trait> $trait for List<T> {
            fn $fn_name(&self) -> String {
                let len = self.items.len();
                if len == 0 {
                    return String::new();
                }
                let last_index = len - 1;

                self.items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let str = match item {
                            ListItem::Trailing { item, separator } => {
                                format!("{}{}", item.$fn_name(), separator.$fn_name())
                            }
                            ListItem::NonTrailing(item) => item.$fn_name(),
                        };

                        if i == last_index {
                            str
                        } else {
                            str.trim_end().to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join($join_with)
            }
        }
    };
}

impl_list!(HasRawValue, get_raw_value, " ");
impl_list!(Print, print, "");
