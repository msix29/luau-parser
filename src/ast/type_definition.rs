use std::fmt::Display;

use super::AstNode;

#[derive(Clone, Debug)]
pub struct TypeDefinition {
    type_name: String,
    is_exported: bool,
}

impl Default for TypeDefinition {
    fn default() -> Self {
        TypeDefinition {
            type_name: "".to_string(),
            is_exported: false,
        }
    }
}

impl Display for TypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl AstNode for TypeDefinition {
    fn get_raw_value(&self) -> String {
        let start = if self.type_name.is_empty() {
            String::from("")
        } else {
            format!("type {} = ", self.type_name)
        };

        format!("{}{}", start, "<PLACEHOLDER_VALUE>")
    }

    #[allow(unused_variables)]
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>> {
        if node.kind() != "TypeDeclaration" {
            return None;
        }

        None
    }
}
