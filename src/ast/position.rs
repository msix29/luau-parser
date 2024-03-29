//! Implements helper traits for positions.

use std::ops::AddAssign;

use crate::prelude::Location;

impl<T: AddAssign> Location<T> {
    /// Offsets the whole location by lines and characters. For offsetting either start
    /// or end, call `offset` of either of them, ex. `location.start.offset(0, 0)`
    pub fn offset<L, C>(&mut self, lines: L, characters: C)
    where
        L: Into<T> + Copy,
        C: Into<T> + Copy,
    {
        self.start.offset(lines, characters);
        self.end.offset(lines, characters);
    }
}
