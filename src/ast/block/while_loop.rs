//! Implements helper traits for while loops.

use crate::{
    prelude::{DoBlock, Expression, HasLocation, Location, LuauStatement, SingleToken, WhileLoop},
    utils::get_location_from_boundaries,
};

impl LuauStatement for WhileLoop {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "whileLoop" {
            return None;
        }

        Some(WhileLoop {
            while_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            condition: Expression::from((node.child(1).unwrap(), code_bytes)),
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}

impl HasLocation for WhileLoop {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.while_keyword.get_location(),
            self.do_block.get_location(),
        )
    }
}
