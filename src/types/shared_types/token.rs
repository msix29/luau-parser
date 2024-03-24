/// A struct represnting a single token, aka a single word.
#[derive(Clone)]
pub struct SingleToken {
    /// All the spaces before the word.
    pub spaces_before: String,

    /// The actual word itself, excluding any leading or trailing spaces.
    pub word: String,

    /// All the spaces after the word.
    pub spaces_after: String,
}
