//! Implements helper traits for numerical for loops.

use crate::{
    prelude::{
        DoBlock, Expression, HasLocation, Location, LuauStatement, NormalizedName, NumericalFor,
        SingleToken,
    },
    utils::get_location_from_boundaries,
};

impl LuauStatement for NumericalFor {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "numericalFor" {
            return None;
        }

        Some(NumericalFor {
            for_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            variable: NormalizedName::from((node.child(1).unwrap(), code_bytes)),
            equal_keyword: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            start: Expression::from((node.child(3).unwrap(), code_bytes)),
            start_comma: SingleToken::from((node.child(4).unwrap(), code_bytes)),
            end: Expression::from((node.child(5).unwrap(), code_bytes)),
            end_comma: node
                .child(6)
                .map(|node| SingleToken::from((node, code_bytes))),
            step: node
                .child(7)
                .map(|node| Expression::from((node, code_bytes))),
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}

impl HasLocation for NumericalFor {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.for_keyword.get_location(),
            self.do_block.get_location(),
        )
    }
}
