//! Location-related items.

use crate::prelude::Position;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing the location of any item.
pub struct Location {
    /// Start _[position](Position)_ of the node.
    pub start: Position,

    /// End _[position](Position)_ of the node.
    pub end: Position,
}
