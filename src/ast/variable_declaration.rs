//! Implements helper trait for _[variable declarations](VariableDeclaration)_.

use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use crate::prelude::{
    AstNode, Expression, ExpressionInner, HasRawValue, List, ListItem, NormalizedName, Print,
    SingleToken, VariableDeclaration,
};

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for VariableDeclaration {
    fn get_raw_value(&self) -> String {
        todo!()
        // let local = self
        //     .local_token
        //     .as_ref()
        //     .map(|token| token.print_leading())
        //     .unwrap_or("".to_string());

        // format!("{} {}", local, self.variable_name)
    }
}

impl Print for VariableDeclaration {
    fn print(&self) -> String {
        todo!()
        // let local = self
        //     .local_token
        //     .as_ref()
        //     .map(|token| token.print())
        //     .unwrap_or("".to_string());

        // format!(
        //     "{}{}{}{}",
        //     local,
        //     self.variable_name.print_trailing(),
        //     self.equal_token
        //         .as_ref()
        //         .map(|token| token.print())
        //         .unwrap_or("".to_string()),
        //     self.expressions
        // )
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

        let expressions =
            ExpressionInner::from_nodes(node.children_by_field_name("value", cursor), code_bytes)
                .to::<Expression, Node<'_>>(node);

        let all_names = node
            .children_by_field_name("binding", cursor)
            .collect::<Vec<Node>>();
        let separators = node
            .children_by_field_name("separator", cursor)
            .collect::<Vec<Node>>();

        let names = all_names
            .iter()
            .enumerate()
            .map(|(i, binding)| {
                if let Some(separator) = separators.get(i) {
                    ListItem::Trailing {
                        item: NormalizedName::from((binding.child(0).unwrap(), code_bytes)),
                        separator: SingleToken::from((*separator, code_bytes)),
                    }
                } else {
                    ListItem::NonTrailing(NormalizedName::from((
                        binding.child(0).unwrap(),
                        code_bytes,
                    )))
                }
            })
            .collect::<Vec<ListItem<NormalizedName>>>();

        variables.push(VariableDeclaration {
            local_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            name_list: List { items: names },
            equal_token: node
                .child_by_field_name("equal")
                .map(|equal| SingleToken::from((equal, code_bytes))),
            expressions,
        });

        // for (i, binding) in bindings.step_by(2).enumerate() {
        //     let expression = if let Some(expression) = expressions.get(i) {
        //         expression.clone()
        //     } else {
        //         Arc::new(ExpressionInner::from(("nil", node)))
        //     };

        //     variables.push(VariableDeclaration {
        //         local_token: if i == 0 {
        //             // Only the first variable has the keyword "local" before it.
        //             Some(SingleToken::from((node.child(0).unwrap(), code_bytes)))
        //         } else {
        //             None
        //         },
        //         equal_token: if i == 0 {
        //             // Only the first variable has the equal sign.
        //             node.child_by_field_name("equal")
        //                 .map(|equal| SingleToken::from((equal, code_bytes)))
        //         } else {
        //             None
        //         },
        //         variable_name: Arc::new(NormalizedName::from((
        //             binding.child(0).unwrap(),
        //             code_bytes,
        //         ))),
        //         expressions: Arc::new(Expression::from((expression, node))),
        //         location: get_location(node),
        //     });
        // }
        Some(variables)
    }
}
