//! Implements helper traits for positions.

use crate::prelude::{Range, Position};

impl Position {
    /// Create a new [`position`](Position).
    pub fn new(line: u32, character: u32) -> Self {
        Self { character, line }
    }

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

    /// Checks whether or not this position is inside the passed range.
    pub fn is_in_bounds(&self, range: &Range) -> bool {
        self.is_after(&range.start) & self.is_before(&range.end)
    }

    /// Checks whether or not this position is after the passed position
    pub fn is_after(&self, position: &Position) -> bool {
        self.line > position.line
            || position.line == self.line && self.character >= position.character
    }

    /// Checks whether or not this position is before the passed position
    pub fn is_before(&self, position: &Position) -> bool {
        self.line < position.line
            || position.line == self.line && self.character <= position.character
    }
}
