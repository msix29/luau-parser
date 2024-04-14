//! Implements helper traits for [`ranges`](Range).

use crate::prelude::{Range, Position};

impl Range {
    /// Create a new [`range`](Range).
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a new [`range`](Range).
    pub fn new2(start_line: u32, start_character: u32, end_line: u32, end_character: u32) -> Self {
        Self {
            start: Position::new(start_line, start_character),
            end: Position::new(end_line, end_character),
        }
    }

    /// Offsets the whole range by lines and characters. For offsetting either start
    /// or end, call `offset` of either of them, ex. `range.start.offset(0, 0)`
    pub fn offset(&mut self, lines: i32, characters: i32) {
        self.start.offset(lines, characters);
        self.end.offset(lines, characters);
    }
}
