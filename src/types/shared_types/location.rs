//! Location-related items.

use crate::prelude::Position;

#[derive(Clone, Debug, Default)]
/// A struct representing the location of an _[AstNode](crate::prelude::AstNode)_.
pub struct Location<Type = u16> {
    /// Start _[position](Position)_ of the node.
    pub start: Position<Type>,

    /// End _[position](Position)_ of the node.
    pub end: Position<Type>,
}
