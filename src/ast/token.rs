use std::fmt::{Debug, Display};

use tree_sitter::Node;

use crate::{
    impl_print,
    prelude::{HasRawValue, SingleToken},
    utils::{get_location, get_spaces},
};

impl HasRawValue for SingleToken {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
impl Display for SingleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl Debug for SingleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Word ({})", &self.get_raw_value()))
    }
}

impl From<(Node<'_>, &[u8])> for SingleToken {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let word = node.utf8_text(code_bytes).unwrap().to_string();
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            word,
            spaces_after,
            location: get_location(node),
        }
    }
}

impl_print!(SingleToken, word);
