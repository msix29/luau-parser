//! Location-related items.

use crate::prelude::Position;

#[derive(Clone, Debug, Default, Copy)]
/// A struct representing the location of any item.
pub struct Location {
    /// Start _[position](Position)_ of the node.
    pub start: Position,

    /// End _[position](Position)_ of the node.
    pub end: Position,
}
