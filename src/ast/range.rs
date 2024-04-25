//! Implements helper traits for [`ranges`](Range).

use crate::prelude::{Position, Range};

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

    /// Turns the range into a [`string`](String), which can be passed to [`from_string`](Self::from_string).
    pub fn serialize_to_string(&self) -> String {
        format!(
            "{},{},{},{}",
            self.start.line, self.start.character, self.end.line, self.end.character
        )
    }

    /// Creates a new [`range`](Range) from the passed [`string`](str).
    ///
    /// # Note
    ///
    /// This function assumes the string is valid and in the bounds of [`u32`]s. Meaning
    /// that if the string is malformed, it'll error. To ensure correctness, use
    /// [`serialize_to_string`](Self::serialize_to_string).
    pub fn from_string(str: &str) -> Self {
        let splits = str
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        Self::new2(splits[0], splits[1], splits[2], splits[3])
    }
}
