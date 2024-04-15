//! Implements helper traits for local and global functions.

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        parse_block, GlobalFunction, GlobalFunctionName, HasRange, List, LocalFunction,
        LuauStatement, Range, SingleToken,
    },
    utils::get_range_from_boundaries,
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
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
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
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
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
