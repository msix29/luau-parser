//! Implemens helper traits for for-in loops.

use crate::{
    prelude::{
        AstNode, DoBlock, Expression, ExpressionInner, GenericFor, HasLocation, List,
        NormalizedName, SingleToken,
    },
    utils::get_location_from_boundaries,
};

impl AstNode for GenericFor {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "forIn" {
            return None;
        }

        Some(GenericFor {
            for_keyword: SingleToken::from((node.child_by_field_name("for").unwrap(), code_bytes)),
            names: List::from_iter(
                node.children_by_field_name("binding", cursor),
                node,
                "separator",
                code_bytes,
                |_, binding| NormalizedName::from((binding.child(0).unwrap(), code_bytes)),
            ),
            in_keyword: SingleToken::from((node.child_by_field_name("in").unwrap(), code_bytes)),
            expressions: ExpressionInner::from_nodes(
                node.children_by_field_name("value", cursor),
                code_bytes,
            )
            .to::<Expression>(),
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}

impl HasLocation for GenericFor {
    fn get_location(&self) -> crate::prelude::Location {
        get_location_from_boundaries(
            self.for_keyword.get_location(),
            self.do_block.get_location(),
        )
    }
}
