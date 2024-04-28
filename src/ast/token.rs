//! Implements a helper `From<>` trait for `SingleToken`.

use smol_str::SmolStr;
use tree_sitter::Node;

use crate::{
    prelude::{HasRange, Range, Token, Trivia},
    utils::{get_range, get_text_from_bytes, get_trivia},
};

impl From<(Node<'_>, &[u8])> for Token {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let word = get_text_from_bytes(code_bytes, node.start_byte(), node.end_byte());
        let (spaces_before, spaces_after) = get_trivia(node, code_bytes);

        Self {
            leading_trivia: spaces_before,
            word: word.into(),
            trailing_trivia: spaces_after,
            range: get_range(node),
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Self {
            leading_trivia: Vec::new(),
            word: value.into(),
            trailing_trivia: Vec::new(),
            range: Range::default(),
        }
    }
}

impl HasRange for &Token {
    fn get_range(&self) -> Range {
        self.range
    }
}
impl HasRange for Token {
    fn get_range(&self) -> Range {
        self.range
    }
}
impl Token {
    /// Create a new single token from the passed word with no spaces and
    /// range at 0, 0 to 0, 0.
    pub fn new(word: &str) -> Self {
        Self {
            leading_trivia: Vec::new(),
            word: word.into(),
            trailing_trivia: Vec::new(),
            ..Default::default()
        }
    }

    /// Create a new [`SingleToken`] with the same everything and only different
    /// range. The old one is dropped.
    pub fn set_range(self, range: Range) -> Self {
        Self {
            leading_trivia: self.leading_trivia,
            word: self.word,
            trailing_trivia: self.trailing_trivia,
            range,
        }
    }

    /// Create a new single token with the same everything as the current one except spaces
    /// before and after.
    pub fn with_spaces(self, spaces_before: &str, spaces_after: &str) -> Self {
        Self {
            leading_trivia: vec![Trivia::Spaces(SmolStr::new(spaces_before))],
            word: self.word,
            trailing_trivia: vec![Trivia::Spaces(SmolStr::new(spaces_after))],
            range: self.range,
        }
    }
}
