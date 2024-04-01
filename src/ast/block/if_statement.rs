//! Implements helper traits for if statements

use std::sync::Arc;

use crate::{
    call_any,
    prelude::{
        parse_block, Ast, ElseIfStatement, ElseStatement, Expression, HasLocation, IfStatement,
        Location, LuauStatement, SingleToken,
    },
    utils::get_location_from_boundaries,
};

impl LuauStatement for IfStatement {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "ifStatement" {
            return None;
        }

        let else_if_expressions = node
            .children_by_field_name("elseif_clause", &mut node.walk())
            .map(|elseif| ElseIfStatement {
                elseif_keyword: SingleToken::from((elseif.child(0).unwrap(), code_bytes)),
                condition: Arc::new(Expression::from((elseif.child(1).unwrap(), code_bytes))),
                then_keyword: SingleToken::from((elseif.child(2).unwrap(), code_bytes)),
                body: elseif.child(3).map_or_else(Ast::default, |body| Ast {
                    tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                    uri: None,
                }),
            })
            .collect::<Vec<ElseIfStatement>>();
        let else_expression = node
            .child_by_field_name("else_clause")
            .map(|node| ElseStatement {
                else_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                body: node.child(2).map_or_else(Ast::default, |body| Ast {
                    tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                    uri: None,
                }),
            });

        Some(IfStatement {
            if_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
            then_keyword: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            body: node.child(3).map_or_else(Ast::default, |body| Ast {
                tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                uri: None,
            }),
            else_if_expressions,
            else_expression,
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
        })
    }
}

impl HasLocation for IfStatement {
    fn get_location(&self) -> Location {
        let end = if let Some(else_if_statement) = self.else_if_expressions.first() {
            else_if_statement.elseif_keyword.get_location()
        } else {
            call_any!(get_location, self.end_keyword, self.else_expression)
        };

        get_location_from_boundaries(self.if_keyword.get_location(), end)
    }
}
impl HasLocation for ElseIfStatement {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.elseif_keyword.get_location(),
            call_any!(get_location, self.then_keyword, self.body.tokens.last()),
        )
    }
}
impl HasLocation for ElseStatement {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.else_keyword.get_location(),
            call_any!(get_location, self.else_keyword, self.body.tokens.last()),
        )
    }
}
