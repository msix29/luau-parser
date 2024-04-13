//! Position

/// A struct representing a specific point in a document. Lines and characters are
///     zero-based.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// The line in which this position points to, starting from 0.
    pub line: u32,

    /// The character in the [`line`](Position::line) that this position points to,
    /// starting from 0.
    pub character: u32,
}
