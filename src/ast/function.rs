//! Implements helper traits for local and global functions.

use std::sync::Arc;

use tree_sitter::{Node, TreeCursor};

use crate::prelude::{
    parse_block, Ast, GlobalFunction, GlobalFunctionName, List, LocalFunction, LuauStatement,
    SingleToken,
};

use super::type_definition::functions::{build_function_parameters, build_function_returns};

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
            local_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            function_keyword: SingleToken::from((node.child(1).unwrap(), code_bytes)),
            function_name: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            opening_parenthesis: SingleToken::from((
                node.child_by_field_name("opening_parenthesis").unwrap(),
                code_bytes,
            )),

            parameters: build_function_parameters(node, code_bytes, false),
            closing_parenthesis: SingleToken::from((
                node.child_by_field_name("closing_parenthesis").unwrap(),
                code_bytes,
            )),
            returns: build_function_returns(node, code_bytes),
            body: node
                .child_by_field_name("body")
                .map(|body| Ast {
                    tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                    uri: None,
                })
                .unwrap_or_default(),
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
        })
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
            function_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            function_name: if let Some(name) = node.child_by_field_name("name") {
                GlobalFunctionName::SimpleName(SingleToken::from((name, code_bytes)))
            } else {
                GlobalFunctionName::Table {
                    table: SingleToken::from((
                        node.child_by_field_name("table").unwrap(),
                        code_bytes,
                    )),
                    keys: List::from_iter(
                        node.children_by_field_name("index", &mut node.walk()),
                        node,
                        "dot",
                        code_bytes,
                        |_, name| SingleToken::from((name, code_bytes)),
                    ),
                    method: node
                        .child_by_field_name("method")
                        .map(|method| SingleToken::from((method, code_bytes))),
                }
            },
            opening_parenthesis: SingleToken::from((
                node.child_by_field_name("opening_parenthesis").unwrap(),
                code_bytes,
            )),
            parameters: build_function_parameters(node, code_bytes, false),
            closing_parenthesis: SingleToken::from((
                node.child_by_field_name("closing_parenthesis").unwrap(),
                code_bytes,
            )),
            returns: build_function_returns(node, code_bytes),
            body: node
                .child_by_field_name("body")
                .map(|body| Ast {
                    tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                    uri: None,
                })
                .unwrap_or_default(),
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
        })
    }
}
