//! Implements helper trait for _[variable declarations](LocalAssignment)_.

use tree_sitter::{Node, TreeCursor};

use crate::prelude::{
    AstNode, Expression, ExpressionInner, List, ListItem, LocalAssignment, NormalizedName,
    SingleToken,
};

impl AstNode for LocalAssignment {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "variableDeclaration" {
            return None;
        }

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

        Some(LocalAssignment {
            local_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            name_list: List { items: names },
            equal_token: node
                .child_by_field_name("equal")
                .map(|equal| SingleToken::from((equal, code_bytes))),
            expressions,
        })
    }
}
