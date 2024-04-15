use tree_sitter::Node;

use crate::prelude::{Expression, LastStatement, SingleToken};

impl From<(Node<'_>, &[u8])> for LastStatement {
    fn from((node, code_bytes): (Node, &[u8])) -> Self {
        let semicolon = node
            .child_by_field_name("semicolon")
            .map(|semicolon| SingleToken::from((semicolon, code_bytes)));

        let node = node.child(0).unwrap();

        match node.kind() {
            "break" => Self::Break((SingleToken::from((node, code_bytes)), semicolon)),
            "continue" => Self::Continue((SingleToken::from((node, code_bytes)), semicolon)),
            "return_statement" => Self::Return {
                return_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                expressions: Expression::from_nodes(
                    node.children_by_field_name("expressions", &mut node.walk()),
                    code_bytes,
                ),
                semicolon,
            },
            _ => unreachable!(),
        }
    }
}
