//! Implements helper trait for _[variable declarations](VariableDeclaration)_.

use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use crate::prelude::{
    AstNode, HasRawValue, NormalizedName, SingleToken, TypeDefinition, Value, VariableDeclaration,
};

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for VariableDeclaration {
    fn get_raw_value(&self) -> String {
        format!("local {}: {}", self.variable_name, self.variable_type)
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

        let mut variables = Vec::new();

        // We do this to free `cursor` so it can be used under it
        let _temp = node
            .children_by_field_name("names", cursor)
            .collect::<Vec<Node>>();
        let bindings = _temp.iter();

        let values = Value::from_nodes(node.children_by_field_name("values", cursor), code_bytes);

        for (i, binding) in bindings.step_by(2).enumerate() {
            let (value, r#type) = if let Some(value) = values.get(i) {
                value.clone()
            } else {
                (Value::from("nil"), Some(TypeDefinition::from("nil")))
            };

            variables.push(VariableDeclaration {
                local_token: if i == 0 {
                    // Only the first variable has the keyword "local" before it.
                    Some(SingleToken::from((node.child(0).unwrap(), code_bytes)))
                } else {
                    None
                },
                variable_name: NormalizedName::from((binding.child(0).unwrap(), code_bytes)),
                variable_type: r#type.unwrap_or(TypeDefinition::default()),
                variable_value: value,
            });
        }
        Some(variables)
    }
}
