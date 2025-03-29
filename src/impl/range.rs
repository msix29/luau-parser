use luau_lexer::prelude::Position;

use crate::types::Range;

impl Range {
    #[inline]
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
