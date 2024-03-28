//! Implements helper traits for if statements

use std::sync::Arc;

use crate::{
    prelude::{
        parse_block, Ast, AstNode, ElseIfStatement, ElseStatement, Expression, IfStatement,
        SingleToken,
    },
    utils::get_location,
};

impl AstNode for IfStatement {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "ifStatement" {
            return None;
        }

        Some(IfStatement {
            if_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
            then_keyword: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            body: node.child(3).map_or_else(Ast::default, |body| Ast {
                tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                uri: None,
            }),
            else_if_expressions: node
                .children_by_field_name("elseif_clause", &mut node.walk())
                .map(|elseif| ElseIfStatement {
                    elseif_keyword: SingleToken::from((elseif.child(0).unwrap(), code_bytes)),
                    condition: Arc::new(Expression::from((elseif.child(1).unwrap(), code_bytes))),
                    then_keyword: SingleToken::from((elseif.child(2).unwrap(), code_bytes)),
                    body: elseif.child(3).map_or_else(Ast::default, |body| Ast {
                        tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                        uri: None,
                    }),
            location: get_location(elseif),
                })
                .collect::<Vec<ElseIfStatement>>(),
            else_expression: node
                .child_by_field_name("else_clause")
                .map(|node| ElseStatement {
                    else_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                    body: node.child(2).map_or_else(Ast::default, |body| Ast {
                        tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                        uri: None,
                    }),
                    location: get_location(node),
                }),
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
            location: get_location(node),
        })
    }
}
