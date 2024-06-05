//! Just a single token that can't be broke down to smaller tokens.

use smol_str::SmolStr;

use super::Range;
use crate::generate_derives;

generate_derives! {
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
    pub struct Comment(pub SmolStr);
}

generate_derives! {
    /// An enum representing items that can surround a [`Token`].
    pub enum Trivia {
        /// Just spaces. Be it new lines, tabs, white spaces, etc.
        Spaces(SmolStr),

        /// A comment
        Comment(Comment),
    }
}

generate_derives! {
    Default,
    /// A struct representing a single token, aka a single word.
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
}
