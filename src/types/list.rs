//! A list made of any length of trailing items and optionally ending with a non-trailing
//! item

use crate::generate_derives;

use super::Token;

generate_derives! {
    /// A possible list item of type `T`.
    pub enum ListItem<T> {
        /// A trailing one.
        ///
        /// ```lua
        /// local _ = 1, 2
        /// ```
        ///
        /// `1` is trailing in this case.
        Trailing {
            /// The actual item.
            item: T,

            /// The separator trailing after it.
            separator: Token,
        },

        /// A non trailing one.
        ///
        /// ```lua
        /// local _ = 1, 2
        /// ```
        ///
        /// `2` is non trailing in this case.
        NonTrailing(T),
    }
}

generate_derives! {
    /// A list holding [`list items`](ListItem) of type `T`.
    pub struct List<T> {
        /// The actual items being stored.
        pub items: Vec<ListItem<T>>,
    }
}
