//! Implements helper trait for _[variable declarations](VariableDeclaration)_.

use std::{fmt::Display, sync::Arc};
use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        AstNode, Expression, ExpressionInner, HasRawValue, NormalizedName, Print, SingleToken,
        VariableDeclaration,
    },
    utils::get_location,
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
            .map(|token| token.print_leading())
            .unwrap_or("".to_string());

        format!("{} {}", local, self.variable_name)
    }
}

impl Print for VariableDeclaration {
    fn print(&self) -> String {
        let local = self
            .local_token
            .as_ref()
            .map(|token| token.print())
            .unwrap_or("".to_string());

        format!(
            "{}{}{}{}",
            local,
            self.variable_name.print_trailing(),
            self.equal_token
                .as_ref()
                .map(|token| token.print())
                .unwrap_or("".to_string()),
            self.variable_value
        )
    }
    fn print_leading(&self) -> String {
        todo!()
    }
    fn print_trailing(&self) -> String {
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

        let _temp = node
            .children_by_field_name("names", cursor)
            .collect::<Vec<Node>>();
        let bindings = _temp.iter();

        let expressions =
            ExpressionInner::from_nodes(node.children_by_field_name("values", cursor), code_bytes);

        for (i, binding) in bindings.step_by(2).enumerate() {
            let expression = if let Some(expression) = expressions.get(i) {
                expression.clone()
            } else {
                Arc::new(ExpressionInner::from(("nil", node)))
            };

            variables.push(VariableDeclaration {
                local_token: if i == 0 {
                    // Only the first variable has the keyword "local" before it.
                    Some(SingleToken::from((node.child(0).unwrap(), code_bytes)))
                } else {
                    None
                },
                equal_token: if i == 0 {
                    // Only the first variable has the equal sign.
                    node.child_by_field_name("equal")
                        .map(|equal| SingleToken::from((equal, code_bytes)))
                } else {
                    None
                },
                variable_name: Arc::new(NormalizedName::from((
                    binding.child(0).unwrap(),
                    code_bytes,
                ))),
                variable_value: Arc::new(Expression::from((expression, node))),
                location: get_location(node),
            });
        }
        Some(variables)
    }
}
