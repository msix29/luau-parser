//! Implements helper trait for _[variable declarations](VariableDeclaration)_.

use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use crate::prelude::{
    AstNode, HasRawValue, NormalizedName, PrettyPrint, SingleToken, Value, VariableDeclaration,
};

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for VariableDeclaration {
    fn get_raw_value(&self) -> String {
        let local = self
            .local_token
            .as_ref()
            .map(|token| token.pretty_print_leading())
            .unwrap_or("".to_string());

        format!("{} {}", local, self.variable_name)
    }
}

impl PrettyPrint for VariableDeclaration {
    fn pretty_print(&self) -> String {
        let local = self
            .local_token
            .as_ref()
            .map(|token| token.pretty_print())
            .unwrap_or("".to_string());

        format!(
            "{}{}{}{}",
            local,
            self.variable_name.pretty_print_trailing(),
            self.equal_token
                .as_ref()
                .map(|token| token.pretty_print())
                .unwrap_or("".to_string()),
            self.variable_value
        )
    }
    fn pretty_print_leading(&self) -> String {
        todo!()
    }
    fn pretty_print_trailing(&self) -> String {
        todo!()
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
            let value = if let Some(value) = values.get(i) {
                value.clone()
            } else {
                Value::from("nil")
            };

            variables.push(VariableDeclaration {
                local_token: if i == 0 {
                    // Only the first variable has the keyword "local" before it.
                    Some(SingleToken::from((node.child(0).unwrap(), code_bytes)))
                } else {
                    None
                },
                equal_token: if i == 0 {
                    // Only the first variable has the requal sign.
                    node.child_by_field_name("equal")
                        .map(|equal| SingleToken::from((equal, code_bytes)))
                } else {
                    None
                },
                variable_name: NormalizedName::from((binding.child(0).unwrap(), code_bytes)),
                variable_value: value,
            });
        }
        Some(variables)
    }
}
