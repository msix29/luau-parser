//! Helps with parsing prefix expressions.

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{
        Expression, ExpressionWrap, FunctionArguments, FunctionCall, FunctionCallInvoked,
        HasRange, Range, LuauStatement, PrefixExp, SingleToken, TableAccess, TableAccessKey,
        TableAccessPrefix, TableKey, Var,
    },
    utils::get_range_from_boundaries,
};

use super::build_table;

/// Extracts data for a table access from a node representing one.
fn handle_table_var(node: Node, code_bytes: &[u8]) -> TableAccess {
    let table_node = node.child_by_field_name("table").unwrap_or(node);
    let prefix = match table_node.kind() {
        "functionCall" => {
            TableAccessPrefix::FunctionCall(Arc::new(handle_function_call(table_node, code_bytes)))
        }
        "exp_wrap" => match handle_prefix_exp(table_node, code_bytes) {
            PrefixExp::ExpressionWrap(value) => TableAccessPrefix::ExpressionWrap(Arc::new(value)),
            _ => unreachable!("This'll always evaluate to a wrap."),
        },
        _ => TableAccessPrefix::Name(SingleToken::from((table_node, code_bytes))),
    };

    TableAccess {
        prefix,
        accessed_keys: node
            .children_by_field_name("key", &mut node.walk())
            .map(|key| match key.kind() {
                //TODO:
                "field_named" => TableAccessKey::Name {
                    dot: SingleToken::from((key.child(0).unwrap(), code_bytes)),
                    name: SingleToken::from((key.child(1).unwrap(), code_bytes)),
                },
                "field_indexed" => TableAccessKey::Expression(TableKey::Expression {
                    open_square_brackets: SingleToken::from((key.child(0).unwrap(), code_bytes)),
                    expression: Arc::new(Expression::from((key.child(1).unwrap(), code_bytes))),
                    close_square_brackets: SingleToken::from((key.child(2).unwrap(), code_bytes)),
                }),
                _ => unreachable!("Key can't be anything else. Got {}", key.to_sexp()),
            })
            .collect(),
    }
}

/// Extracts data for a function call from a node representing one.
fn handle_function_call(prefix_exp: Node, code_bytes: &[u8]) -> FunctionCall {
    let invoked = if let Some(invoked) = prefix_exp.child_by_field_name("invoked") {
        FunctionCallInvoked::Function(Arc::new(handle_prefix_exp(invoked, code_bytes)))
    } else {
        FunctionCallInvoked::TableMethod {
            table: Arc::new(handle_prefix_exp(
                prefix_exp.child_by_field_name("table").unwrap(),
                code_bytes,
            )),
            colon: SingleToken::from((
                prefix_exp.child_by_field_name("colon").unwrap(),
                code_bytes,
            )),
            method: SingleToken::from((
                prefix_exp.child_by_field_name("method").unwrap(),
                code_bytes,
            )),
        }
    };

    let arguments_node = prefix_exp.child_by_field_name("arguments").unwrap();
    let arguments = match arguments_node.kind() {
        "table" => FunctionArguments::Table(build_table(prefix_exp, code_bytes)),
        "string" => FunctionArguments::String(SingleToken::from((arguments_node, code_bytes))),
        _ => {
            let arguments = Expression::from_nodes(
                arguments_node.children_by_field_name("arguments", &mut arguments_node.walk()),
                code_bytes,
            )
            .to::<Expression>();

            FunctionArguments::List {
                open_parenthesis: SingleToken::from((
                    arguments_node
                        .child_by_field_name("open_parenthesis")
                        .unwrap(),
                    code_bytes,
                )),
                arguments,
                close_parenthesis: SingleToken::from((
                    arguments_node
                        .child_by_field_name("close_parenthesis")
                        .unwrap(),
                    code_bytes,
                )),
            }
        }
    };

    FunctionCall { invoked, arguments }
}

/// Extracts needed information from a node representing any possible prefix expression.
pub(crate) fn handle_prefix_exp(prefix_exp: Node, code_bytes: &[u8]) -> PrefixExp {
    match prefix_exp.kind() {
        "var" => {
            // let node = prefix_exp.child(0).unwrap();
            if prefix_exp.child_count() == 1 {
                PrefixExp::Var(Var::Name(SingleToken::from((prefix_exp, code_bytes))))
            } else {
                PrefixExp::Var(Var::TableAccess(handle_table_var(prefix_exp, code_bytes)))
            }
        }
        "functionCall" => PrefixExp::FunctionCall(handle_function_call(prefix_exp, code_bytes)),
        "exp_wrap" => PrefixExp::ExpressionWrap(ExpressionWrap {
            opening_parenthesis: SingleToken::from((prefix_exp.child(0).unwrap(), code_bytes)),
            expression: Arc::new(Expression::from((prefix_exp.child(1).unwrap(), code_bytes))),
            closing_parenthesis: SingleToken::from((prefix_exp.child(2).unwrap(), code_bytes)),
        }),
        _ => panic!("This shouldn't be reached."),
    }
}

impl LuauStatement for FunctionCall {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "functionCall" {
            return None;
        }

        Some(handle_function_call(node, code_bytes))
    }
}

impl HasRange for TableAccess {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.prefix.get_range(),
            // There's at least one key.
            self.accessed_keys.last().unwrap().get_range(),
        )
    }
}
impl HasRange for TableAccessPrefix {
    fn get_range(&self) -> Range {
        match self {
            TableAccessPrefix::Name(value) => value.get_range(),
            TableAccessPrefix::FunctionCall(value) => value.get_range(),
            TableAccessPrefix::ExpressionWrap(value) => value.get_range(),
        }
    }
}
impl HasRange for TableAccessKey {
    fn get_range(&self) -> Range {
        match self {
            TableAccessKey::Expression(value) => value.get_range(),
            TableAccessKey::Name { dot, name } => {
                get_range_from_boundaries(dot.get_range(), name.get_range())
            }
        }
    }
}

impl HasRange for FunctionCall {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.invoked.get_range(), self.arguments.get_range())
    }
}
impl HasRange for FunctionArguments {
    fn get_range(&self) -> Range {
        match self {
            FunctionArguments::String(value) => value.get_range(),
            FunctionArguments::Table(value) => value.get_range(),
            FunctionArguments::List {
                open_parenthesis,
                arguments: _,
                close_parenthesis,
            } => get_range_from_boundaries(
                open_parenthesis.get_range(),
                close_parenthesis.get_range(),
            ),
        }
    }
}
impl HasRange for FunctionCallInvoked {
    fn get_range(&self) -> Range {
        match self {
            FunctionCallInvoked::Function(value) => value.get_range(),
            FunctionCallInvoked::TableMethod {
                table,
                colon: _,
                method,
            } => get_range_from_boundaries(table.get_range(), method.get_range()),
        }
    }
}

impl HasRange for Var {
    fn get_range(&self) -> Range {
        match self {
            Var::Name(value) => value.get_range(),
            Var::TableAccess(value) => value.get_range(),
        }
    }
}
impl HasRange for ExpressionWrap {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.opening_parenthesis.get_range(),
            self.closing_parenthesis.get_range(),
        )
    }
}

impl HasRange for PrefixExp {
    fn get_range(&self) -> Range {
        match self {
            PrefixExp::Var(value) => value.get_range(),
            PrefixExp::FunctionCall(value) => value.get_range(),
            PrefixExp::ExpressionWrap(value) => value.get_range(),
        }
    }
}
