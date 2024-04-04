//! Implements helper traits for locations.

use crate::prelude::{Location, Position};

impl Location {
    /// Offsets the whole location by lines and characters. For offsetting either start
    /// or end, call `offset` of either of them, ex. `location.start.offset(0, 0)`
    pub fn offset(&mut self, lines: i32, characters: i32) {
        self.start.offset(lines, characters);
        self.end.offset(lines, characters);
    }

    /// Checks whether or not the passed position is inside this location.
    pub fn is_in_bounds(&self, position: &Position) -> bool {
        let is_after_start = self.start.line == position.line
            && self.start.character <= position.character
            || self.start.line < position.line;
        let is_before_end = self.end.line == position.line
            && self.end.character >= position.character
            || self.end.line > position.line;

        is_after_start && is_before_end
    }
}
