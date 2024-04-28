//! Implements display traits for single tokens.

#[cfg(feature = "raw-values")]
use crate::types::HasRawValue;
use crate::types::{Comment, Print, Token, Trivia};

#[cfg(feature = "raw-values")]
impl HasRawValue for Token {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
#[cfg(feature = "raw-values")]
impl HasRawValue for Trivia {
    fn get_raw_value(&self) -> String {
        match self {
            Trivia::Spaces(word) | Trivia::Comment(Comment(word)) => word.to_string(),
        }
    }
}
impl Print for Token {
    fn print(&self) -> String {
        format!(
            "{}{}{}",
            self.leading_trivia.print(),
            self.word,
            self.trailing_trivia.print()
        )
    }
}
impl Print for Trivia {
    fn print(&self) -> String {
        match self {
            Self::Spaces(word) | Self::Comment(Comment(word)) => word.to_string(),
        }
    }
}
