//! Implements helper traits for local and global functions.

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        parse_block, FromNode, GlobalFunction, GlobalFunctionName, HasRange, List, LocalFunction,
        LuauStatement, Range, Token,
    },
    utils::{get_range_from_boundaries, map_option},
};

use super::type_definition::helper_functions::{
    build_function_parameters, build_function_returns, build_generics,
};

impl LuauStatement for LocalFunction {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "localFunction" {
            return None;
        }

        Some(LocalFunction {
            local_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            function_keyword: Token::from_node(node.child(1)?, code_bytes)?,
            function_name: Token::from_node(node.child(2)?, code_bytes)?,
            generics: build_generics(node, code_bytes),
            opening_parenthesis: Token::from_node(
                node.child_by_field_name("opening_parenthesis")?,
                code_bytes,
            )?,

            parameters: build_function_parameters(node, code_bytes, false),
            closing_parenthesis: Token::from_node(
                node.child_by_field_name("closing_parenthesis")?,
                code_bytes,
            )?,
            colon: node
                .child_by_field_name("colon")
                .map(|colon| Token::from_node(colon, code_bytes))?,
            returns: build_function_returns(node, code_bytes),
            body: node
                .child_by_field_name("body")
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: Token::from_node(node.child_by_field_name("end")?, code_bytes)?,
        })
    }
}
impl HasRange for LocalFunction {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.local_keyword.get_range(), self.end_keyword.get_range())
    }
}

impl LuauStatement for GlobalFunction {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "globalFunction" {
            return None;
        }

        Some(GlobalFunction {
            function_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            function_name: if let Some(name) = node.child_by_field_name("name") {
                GlobalFunctionName::SimpleName(Token::from_node(name, code_bytes)?)
            } else {
                GlobalFunctionName::Table {
                    table: Token::from_node(node.child_by_field_name("table")?, code_bytes)?,
                    keys: List::from_iter(
                        node.children_by_field_name("index", &mut node.walk()),
                        node,
                        "_", // Matches nothing.
                        code_bytes,
                        |_, node| {
                            Some((
                                Token::from_node(node.prev_sibling()?, code_bytes)?,
                                Token::from_node(node, code_bytes)?,
                            ))
                        },
                    ),
                    method: map_option(node.child_by_field_name("method"), |method| {
                        let method = method?;

                        Some((
                            Token::from_node(method.prev_sibling()?, code_bytes)?,
                            Token::from_node(method, code_bytes)?,
                        ))
                    }),
                }
            },
            generics: build_generics(node, code_bytes),
            opening_parenthesis: Token::from_node(
                node.child_by_field_name("opening_parenthesis")?,
                code_bytes,
            )?,
            parameters: build_function_parameters(node, code_bytes, false),
            closing_parenthesis: Token::from_node(
                node.child_by_field_name("closing_parenthesis")?,
                code_bytes,
            )?,
            colon: node
                .child_by_field_name("colon")
                .map(|colon| Token::from_node(colon, code_bytes))?,
            returns: build_function_returns(node, code_bytes),
            body: node
                .child_by_field_name("body")
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: Token::from_node(node.child_by_field_name("end")?, code_bytes)?,
        })
    }
}
impl HasRange for GlobalFunction {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.function_keyword.get_range(),
            self.end_keyword.get_range(),
        )
    }
}
