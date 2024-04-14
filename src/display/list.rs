//! Implements display traits for lists.
use crate::prelude::{HasRawValue, List, ListItem, Print};

/// Gets a vector for implementing a specific trait for [`List<T>`]. It'll
/// contain all value inside the self but as a `String`.
macro_rules! impl_list_vec {
    ($fn_name: ident, $self: ident) => {{
        let len = $self.items.len();
        if len == 0 {
            return String::new();
        }
        let last_index = len - 1;

        $self
            .items
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
    }};
}

impl<T: HasRawValue> List<T> {
    /// Gets the raw value but instead of a simple space as a separator,
    /// uses the passed one.
    pub fn raw_value_with_separator(&self, separator: &str) -> String {
        impl_list_vec!(get_raw_value, self).join(separator)
    }
}
impl<T: HasRawValue> HasRawValue for List<T> {
    fn get_raw_value(&self) -> String {
        self.raw_value_with_separator(" ")
    }
}

impl<T: Print> Print for List<T> {
    fn print(&self) -> String {
        impl_list_vec!(print, self).join("")
    }
}
