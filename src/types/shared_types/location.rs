//! Location-related items.

use crate::prelude::Position;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing the location of any item.
pub struct Location {
    /// Start [`position`](Position) of the node.
    pub start: Position,

    /// End [`position`](Position) of the node.
    pub end: Position,
}
