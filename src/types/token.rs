//! Just a single token that can't be broke down to smaller tokens.

use smol_str::SmolStr;

use super::Range;


/// A struct representing a comment. Single line comments:
///
/// ```lua
/// ------ weirdly long
/// -- comment
/// ```
///
/// Multi-line comments.
///
/// ```lua
/// --[[
///     nice
/// ]]
/// ```

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Comment(pub SmolStr);

/// An enum representing items that can surround a [`Token`].
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trivia {
    /// Just spaces. Be it new lines, tabs, white spaces, etc.
    Spaces(SmolStr),

    /// A comment
    Comment(Comment),
}

/// A struct representing a single token, aka a single word.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    /// All the spaces before the word.
    pub leading_trivia: Vec<Trivia>,

    /// The actual word itself, excluding any leading or trailing spaces.
    pub word: SmolStr,

    /// All the spaces after the word.
    pub trailing_trivia: Vec<Trivia>,

    /// Exact range of the word, excluding spaces.
    pub range: Range,
}
