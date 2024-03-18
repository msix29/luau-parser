use crate::prelude::Position;

/// A struct representing the location of an [_AstNode_](crate::prelude::AstNode).
pub struct Location {
    /// Start _[position](Position)_ of the node.
    pub start: Position,

    /// End _[position](Position)_ of the node.
    pub end: Position,
}
