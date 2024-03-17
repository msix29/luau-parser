use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use super::AstNode;

#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    variable_name: String,
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl AstNode for VariableDeclaration {
    fn get_raw_value(&self) -> String {
        format!("local {}", self.variable_name)
    }

    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>> {
        let statement = node.child(0).unwrap();
        if statement.kind() != "variableDeclaration" {
            return None;
        }

        let all_names = statement.child(1).unwrap();
        let mut variables = Vec::new();

        for child in all_names.children(cursor).step_by(2) {
            variables.push(VariableDeclaration {
                variable_name: child.utf8_text(code_bytes).unwrap().to_string(),
            });
        }
        Some(variables)
    }
}
