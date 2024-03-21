use std::fmt::{Debug, Display};

use tree_sitter::Node;

use crate::{
    impl_pretty_print, prelude::{HasRawValue, SingleToken}, utils::get_spaces
};

impl HasRawValue for SingleToken {
    fn get_raw_value(&self) -> String {
        format!("{}{}{}", self.spaces_before, self.word, self.spaces_after)
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
impl Default for SingleToken {
    fn default() -> Self {
        Self {
            spaces_before: "".to_string(),
            word: "".to_string(),
            spaces_after: "".to_string(),
        }
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
        }
    }
}

impl_pretty_print!(SingleToken, word);
