//! Implements a helper `From<>` trait for `SingleToken`.

use smol_str::SmolStr;
use tree_sitter::Node;

use crate::{
    prelude::{Comment, HasRange, Range, Token, Trivia},
    utils::{get_range, get_text_from_bytes, get_trivia, remove_surrounding_pair},
};

impl From<(Node<'_>, &[u8])> for Token {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        if node.is_error() | node.is_missing() {
            return Self::new("*error*");
        }

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

impl Trivia {
    /// Whether or not this trivia is of type [`Comment`](Trivia::Comment).
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }

    /// Whether or not this trivia is of type [`Spaces`](Trivia::Spaces).
    pub fn is_spaces(&self) -> bool {
        matches!(self, Self::Spaces(_))
    }
}

impl Comment {
    /// Gets the text of the comment by removing the comment delimeters.
    pub fn get_text(&self) -> String {
        let trimmed = self.0.trim_start_matches('-');
        if trimmed.starts_with('[') {
            remove_surrounding_pair(remove_surrounding_pair(trimmed).trim_matches('='))
        } else {
            trimmed.to_string()
        }
    }
}
