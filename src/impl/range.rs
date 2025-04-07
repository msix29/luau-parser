//! All impl blocks for [`Range`].

use luau_lexer::prelude::Position;

use crate::types::Range;

impl Range {
    /// Crate a new [`Range`].
    #[inline]
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
