//! Just a single token that can't be broke down to smaller tokens.

use smol_str::SmolStr;

use super::Range;

/// A struct represnting a single token, aka a single word.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    /// All the spaces before the word.
    pub spaces_before: SmolStr,

    /// The actual word itself, excluding any leading or trailing spaces.
    pub word: SmolStr,

    /// All the spaces after the word.
    pub spaces_after: SmolStr,

    /// Exact range of the word, excluding spaces.
    pub range: Range,
}
