//! A list made of any length of trailing items and optionally ending with a non-trailing
//! item

use luau_lexer::prelude::Token;

use crate::types::Bracketed;

/// A possible list item of type `T`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

/// A list holding [`list items`](ListItem) of type `T`.
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct List<T> {
    /// The actual items being stored.
    pub items: Vec<ListItem<T>>,
}

/// A [`list`](List) holding [`list items`](ListItem) of type `T` that must be
/// surrounded by [`brackets`](Token).
pub type BracketedList<T> = Bracketed<List<T>>;
