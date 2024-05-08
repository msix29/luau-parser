//! Implements helper traits for local and global functions.

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        parse_block, GlobalFunction, GlobalFunctionName, HasRange, List, LocalFunction,
        LuauStatement, Range, Token,
    },
    utils::get_range_from_boundaries,
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
            local_keyword: Token::from((node.child(0).unwrap(), code_bytes)),
            function_keyword: Token::from((node.child(1).unwrap(), code_bytes)),
            function_name: Token::from((node.child(2).unwrap(), code_bytes)),
            generics: build_generics(node, code_bytes),
            opening_parenthesis: Token::from((
                node.child_by_field_name("opening_parenthesis").unwrap(),
                code_bytes,
            )),

            parameters: build_function_parameters(node, code_bytes, false),
            closing_parenthesis: Token::from((
                node.child_by_field_name("closing_parenthesis").unwrap(),
                code_bytes,
            )),
            colon: node
                .child_by_field_name("colon")
                .map(|colon| Token::from((colon, code_bytes))),
            returns: build_function_returns(node, code_bytes),
            body: node
                .child_by_field_name("body")
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: Token::from((node.child_by_field_name("end").unwrap(), code_bytes)),
            #[cfg(feature = "lsp-ready")]
            references: Vec::new(),
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
            function_keyword: Token::from((node.child(0).unwrap(), code_bytes)),
            function_name: if let Some(name) = node.child_by_field_name("name") {
                GlobalFunctionName::SimpleName(Token::from((name, code_bytes)))
            } else {
                GlobalFunctionName::Table {
                    table: Token::from((node.child_by_field_name("table").unwrap(), code_bytes)),
                    keys: List::from_iter(
                        node.children_by_field_name("index", &mut node.walk()),
                        node,
                        "_", // Matches nothing.
                        code_bytes,
                        |_, node| {
                            (
                                Token::from((node.prev_sibling().unwrap(), code_bytes)),
                                Token::from((node, code_bytes)),
                            )
                        },
                    ),
                    method: node.child_by_field_name("method").map(|method| {
                        (
                            Token::from((method.prev_sibling().unwrap(), code_bytes)),
                            Token::from((method, code_bytes)),
                        )
                    }),
                }
            },
            generics: build_generics(node, code_bytes),
            opening_parenthesis: Token::from((
                node.child_by_field_name("opening_parenthesis").unwrap(),
                code_bytes,
            )),
            parameters: build_function_parameters(node, code_bytes, false),
            closing_parenthesis: Token::from((
                node.child_by_field_name("closing_parenthesis").unwrap(),
                code_bytes,
            )),
            colon: node
                .child_by_field_name("colon")
                .map(|colon| Token::from((colon, code_bytes))),
            returns: build_function_returns(node, code_bytes),
            body: node
                .child_by_field_name("body")
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: Token::from((node.child_by_field_name("end").unwrap(), code_bytes)),
            #[cfg(feature = "lsp-ready")]
            references: Vec::new(),
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
