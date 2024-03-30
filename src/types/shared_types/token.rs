//! Just a single token that can't be broke down to smaller tokens.

use super::Location;

/// A struct represnting a single token, aka a single word.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleToken {
    /// All the spaces before the word.
    pub spaces_before: String,

    /// The actual word itself, excluding any leading or trailing spaces.
    pub word: String,

    /// All the spaces after the word.
    pub spaces_after: String,

    /// Exact location of the word, excluding spaces.
    pub location: Location,
}
