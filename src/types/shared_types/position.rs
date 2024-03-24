/// A struct representing a specific point in a document. Lines and characters start from 0
/// and ends at `Type`'s upper limit - 1 (defaults to 65535 - 1 in u16, which is the default
/// type). The implementations for _[Position<>]_ are only implemented for all of `i<>`
/// and `u<>`, aka `i16`, `u32`, `isize`, etc.
pub struct Position<Type = u16> {
    /// The line in which this position points to, starting from 0.
    pub line: Type,

    /// The character in the _[line](Position::line)_ that this position points to,
    /// starting from 0.
    pub character: Type,
}
