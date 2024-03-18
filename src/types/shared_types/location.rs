use crate::prelude::Position;

/// A struct representing the location of an _[AstNode](crate::prelude::AstNode)_.
pub struct Location {
    /// Start _[position](Position)_ of the node.
    pub start: Position,

    /// End _[position](Position)_ of the node.
    pub end: Position,
}
