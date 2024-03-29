//! Implements helper traits for locations.

use crate::prelude::Position;

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
}
