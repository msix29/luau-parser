use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use super::{type_definition::TypeDefinition, AstNode, HasRawValue};

#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    variable_name: String,
    variable_type: TypeDefinition,
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for VariableDeclaration {
    fn get_raw_value(&self) -> String {
        format!(
            "local {}: {}",
            self.variable_name,
            self.variable_type.get_raw_value()
        )
    }
}

impl AstNode for VariableDeclaration {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>> {
        if node.kind() != "variableDeclaration" {
            return None;
        }

        let all_names = node.child_by_field_name("names").unwrap();
        let mut variables = Vec::new();

        for child in all_names.children(cursor).step_by(2) {
            variables.push(VariableDeclaration {
                variable_name: child.utf8_text(code_bytes).unwrap().to_string(),
                variable_type: TypeDefinition::default(),
            });
        }
        Some(variables)
    }
}
