//! The [`Range`] struct.

use luau_lexer::prelude::Position;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// A struct representing the range of any item.
pub struct Range {
    /// Start [`position`](Position) of the node.
    pub start: Position,

    /// End [`position`](Position) of the node.
    pub end: Position,
}
