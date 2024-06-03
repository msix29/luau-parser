//! Implements helper traits for if statements

use std::sync::Arc;
use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        parse_block, Ast, ElseIfStatement, ElseStatement, Expression, FromNode, HasRange,
        IfStatement, LuauStatement, Range, Token,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for IfStatement {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "ifStatement" {
            return None;
        }

        let mut else_if_expressions = Vec::new();
        for node in node.children_by_field_name("elseif_clause", &mut node.walk()) {
            else_if_expressions.push(ElseIfStatement::from_node(node, code_bytes)?);
        }

        Some(IfStatement {
            if_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            condition: Expression::from_node(node.child(1)?, code_bytes).map(Arc::new)?,
            then_keyword: Token::from_node(node.child(2)?, code_bytes)?,
            body: node
                .child(3)
                .map_or_else(Ast::default, |body| parse_block(&body, code_bytes, None)),
            else_if_statements: else_if_expressions,
            else_statement: ElseStatement::from_node(node, code_bytes),
            end_keyword: Token::from_node(node.child_by_field_name("end")?, code_bytes)?,
        })
    }
}

impl FromNode for ElseStatement {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        if let Some(node) = node.child_by_field_name("else_clause") {
            Some(ElseStatement {
                else_keyword: Token::from_node(node.child(0)?, code_bytes)?,
                body: node
                    .child(1)
                    .map_or_else(Ast::default, |body| parse_block(&body, code_bytes, None)),
            })
        } else {
            None
        }
    }
}
impl FromNode for ElseIfStatement {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        Some(ElseIfStatement {
            elseif_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            condition: Arc::new(Expression::from_node(node.child(1)?, code_bytes)?),
            then_keyword: Token::from_node(node.child(2)?, code_bytes)?,
            body: node
                .child(3)
                .map_or_else(Ast::default, |body| parse_block(&body, code_bytes, None)),
        })
    }
}

impl HasRange for IfStatement {
    fn get_range(&self) -> Range {
        let end = if let Some(else_if_statement) = self.else_if_statements.first() {
            else_if_statement.elseif_keyword.get_range()
        } else if let Some(else_statement) = &self.else_statement {
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
            if_statement.else_statement.as_ref().map_or_else(
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
