//! Implements helper traits for locations.

use crate::prelude::Location;

impl Location {
    /// Offsets the whole location by lines and characters. For offsetting either start
    /// or end, call `offset` of either of them, ex. `location.start.offset(0, 0)`
    pub fn offset(&mut self, lines: i32, characters: i32) {
        self.start.offset(lines, characters);
        self.end.offset(lines, characters);
    }
}
