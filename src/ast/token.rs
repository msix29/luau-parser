//! Implements a helper `From<>` trait for `SingleToken`.

use tree_sitter::Node;

use crate::{
    prelude::{HasRange, Range, SingleToken},
    utils::{get_range, get_spaces, get_text_from_bytes},
};

impl From<(Node<'_>, &[u8])> for SingleToken {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let word = get_text_from_bytes(code_bytes, node.start_byte(), node.end_byte());
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            word,
            spaces_after,
            range: get_range(node),
        }
    }
}

impl From<&str> for SingleToken {
    fn from(value: &str) -> Self {
        Self {
            spaces_before: "".to_string(),
            word: value.to_string(),
            spaces_after: "".to_string(),
            range: Range::default(),
        }
    }
}

impl HasRange for &SingleToken {
    fn get_range(&self) -> Range {
        self.range
    }
}
impl HasRange for SingleToken {
    fn get_range(&self) -> Range {
        self.range
    }
}
impl SingleToken {
    /// Create a new single token from the passed word with no spaces and
    /// range at 0, 0 to 0, 0.
    pub fn new(word: &str) -> Self {
        Self {
            spaces_before: "".to_string(),
            word: word.to_string(),
            spaces_after: "".to_string(),
            ..Default::default()
        }
    }

    /// Create a new [`SingleToken`] with the same everything and only different
    /// range. The old one is dropped.
    pub fn set_range(self, range: Range) -> Self {
        Self {
            spaces_before: self.spaces_before,
            word: self.word,
            spaces_after: self.spaces_after,
            range,
        }
    }

    /// Create a new single token with the same everything as the current one except spaces
    /// before and after.
    pub fn with_spaces(self, spaces_before: &str, spaces_after: &str) -> Self {
        Self {
            spaces_before: spaces_before.to_string(),
            word: self.word,
            spaces_after: spaces_after.to_string(),
            range: self.range,
        }
    }
}
