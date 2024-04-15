//! Implements helper traits for if statements

use std::sync::Arc;

use crate::{
    prelude::{
        parse_block, Ast, ElseIfStatement, ElseStatement, Expression, HasRange, IfStatement,
        LuauStatement, Range, SingleToken,
    },
    utils::get_range_from_boundaries,
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
                body: elseif
                    .child(3)
                    .map_or_else(Ast::default, |body| parse_block(&body, code_bytes, None)),
            })
            .collect::<Vec<ElseIfStatement>>();
        let else_expression = node
            .child_by_field_name("else_clause")
            .map(|node| ElseStatement {
                else_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                body: node
                    .child(2)
                    .map_or_else(Ast::default, |body| parse_block(&body, code_bytes, None)),
            });

        Some(IfStatement {
            if_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
            then_keyword: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            body: node
                .child(3)
                .map_or_else(Ast::default, |body| parse_block(&body, code_bytes, None)),
            else_if_expressions,
            else_expression,
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
        })
    }
}

impl HasRange for IfStatement {
    fn get_range(&self) -> Range {
        let end = if let Some(else_if_statement) = self.else_if_expressions.first() {
            else_if_statement.elseif_keyword.get_range()
        } else if let Some(else_statement) = &self.else_expression {
            else_statement.else_keyword.get_range()
        } else {
            self.end_keyword.get_range()
        };

        get_range_from_boundaries(self.if_keyword.get_range(), end)
    }
}
impl ElseIfStatement {
    /// Get the range of this else if statement.
    pub fn get_range(&self, if_statement: &IfStatement) -> Range {
        get_range_from_boundaries(
            self.elseif_keyword.get_range(),
            if_statement.else_expression.as_ref().map_or_else(
                || if_statement.end_keyword.get_range(),
                |else_statement| else_statement.else_keyword.get_range(),
            ),
        )
    }
}
impl ElseStatement {
    /// Get the range of this else statement.
    pub fn get_range(&self, if_statement: &IfStatement) -> Range {
        get_range_from_boundaries(
            self.else_keyword.get_range(),
            if_statement.end_keyword.get_range(),
        )
    }
}
