//! Position

/// A struct representing a specific point in a document. Lines and characters start from 0.
#[derive(Clone, Debug, Default, Copy)]
pub struct Position {
    /// The line in which this position points to, starting from 0.
    pub line: u32,

    /// The character in the _[line](Position::line)_ that this position points to,
    /// starting from 0.
    pub character: u32,
}
