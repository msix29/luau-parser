//! Implements helper traits for locations.

use std::ops::AddAssign;

use crate::prelude::Position;

impl<T: AddAssign> Position<T> {
    /// Offsets the current position by lines and characters. If you're adding both lines
    /// and characters, making sure to set characters to `0` before calling this function
    /// to ensure correct results.
    pub fn offset<L: Into<T>, C: Into<T>>(&mut self, lines: L, characters: C) {
        self.line += lines.into();
        self.character += characters.into();
    }

    /// Sets line to a specific value.
    pub fn set_line<L: Into<T>>(&mut self, line: L) {
        self.line = line.into();
    }

    /// Sets line to a specific value.
    pub fn set_character<C: Into<T>>(&mut self, character: C) {
        self.character = character.into();
    }
}
