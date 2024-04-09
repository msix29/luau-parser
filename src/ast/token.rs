//! Implements a helper `From<>` trait for `SingleToken`.

use tree_sitter::Node;

use crate::{
    prelude::{HasLocation, Location, SingleToken},
    utils::{get_location, get_spaces, get_text_from_bytes},
};

impl From<(Node<'_>, &[u8])> for SingleToken {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let word = get_text_from_bytes(code_bytes, node.start_byte(), node.end_byte());
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            word,
            spaces_after,
            location: get_location(node),
        }
    }
}

impl From<&str> for SingleToken {
    fn from(value: &str) -> Self {
        Self {
            spaces_before: "".to_string(),
            word: value.to_string(),
            spaces_after: "".to_string(),
            location: Location::default(),
        }
    }
}

impl HasLocation for &SingleToken {
    fn get_location(&self) -> Location {
        self.location
    }
}
impl HasLocation for SingleToken {
    fn get_location(&self) -> Location {
        self.location
    }
}
impl SingleToken {
    /// Create a new single token from the passed word with no spaces and
    /// location at 0, 0.
    pub fn new(word: &str) -> SingleToken {
        Self {
            spaces_before: "".to_string(),
            word: word.to_string(),
            spaces_after: "".to_string(),
            ..Default::default()
        }
    }

    /// Create a new single token from the passed word with passed spaces and
    /// location at 0, 0.
    pub fn new_with_spaces(spaces_before: &str, word: &str, spaces_after: &str) -> SingleToken {
        Self {
            spaces_before: spaces_before.to_string(),
            word: word.to_string(),
            spaces_after: spaces_after.to_string(),
            ..Default::default()
        }
    }
}
