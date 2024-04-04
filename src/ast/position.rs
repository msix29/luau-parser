//! Implements helper traits for positions.

use crate::prelude::{Location, Position};

impl Position {
    /// Offsets the current position by lines and characters. If you're adding both lines
    /// and characters, making sure to set characters to `0` before calling this function
    /// to ensure correct results.
    pub fn offset(&mut self, lines: i32, characters: i32) {
        self.line = self.line.saturating_add_signed(lines);
        self.character = self.character.saturating_add_signed(characters);
    }

    /// Sets line to a specific value.
    pub fn set_line(&mut self, line: u32) {
        self.line = line;
    }

    /// Sets line to a specific value.
    pub fn set_character(&mut self, character: u32) {
        self.character = character;
    }

    /// Checks whether or not this position is inside the passed location.
    pub fn is_in_bounds(&self, location: &Location) -> bool {
        let is_after_start = location.start.line == self.line
            && location.start.character <= self.character
            || location.start.line < self.line;
        let is_before_end = location.end.line == self.line
            && location.end.character >= self.character
            || location.end.line > self.line;

        is_after_start && is_before_end
    }
}
