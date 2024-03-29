//! Helps with parsing prefix expressions.

use std::sync::Arc;

use tree_sitter::Node;

use crate::prelude::{
    Expression, ExpressionInner, ExpressionWrap, FunctionArguments, FunctionCall,
    FunctionCallInvoked, PrefixExp, SingleToken, TableAccess, TableAccessPrefix, TableKey, Var,
};

use super::expression_inner::build_table;

/// Extracts data for a table access from a node representing one.
fn handle_table_var(node: Node, code_bytes: &[u8]) -> TableAccess {
    let table_node = node.child_by_field_name("table").unwrap();
    let prefix = match table_node.kind() {
        "name" => TableAccessPrefix::Name(table_node.utf8_text(code_bytes).unwrap().to_string()),
        "functionCall" => {
            TableAccessPrefix::FunctionCall(Arc::new(handle_function_call(table_node, code_bytes)))
        }
        "exp_wrap" => match handle_prefix_exp(table_node, code_bytes) {
            PrefixExp::ExpressionWrap(value) => TableAccessPrefix::ExpressionWrap(Arc::new(value)),
            _ => unreachable!("This'll always evaluate to a wrap."),
        },
        _ => TableAccessPrefix::TableAccess(Arc::new(handle_table_var(table_node, code_bytes))),
    };

    let key = if let Some(key) = node.child_by_field_name("keyName") {
        TableKey::String(key.utf8_text(code_bytes).unwrap().to_string())
    } else {
        let key = node.child_by_field_name("keyExp").unwrap();

        TableKey::Expression {
            open_square_brackets: SingleToken::from((key.prev_sibling().unwrap(), code_bytes)),
            expression: Arc::new(Expression::from((key, code_bytes))),
            close_square_brackets: SingleToken::from((key.next_sibling().unwrap(), code_bytes)),
        }
    };

    TableAccess {
        prefix,
        last_accessed_key: Arc::new(key),
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
            method: prefix_exp
                .child_by_field_name("method")
                .unwrap()
                .utf8_text(code_bytes)
                .unwrap()
                .to_string(),
        }
    };

    let arguments_node = prefix_exp.child_by_field_name("arguments").unwrap();
    let arguments = match arguments_node.kind() {
        "table" => FunctionArguments::Table(build_table(prefix_exp, code_bytes)),
        "string" => FunctionArguments::String(SingleToken::from((arguments_node, code_bytes))),
        _ => {
            let arguments = ExpressionInner::from_nodes(
                arguments_node.children_by_field_name("arguments", &mut arguments_node.walk()),
                code_bytes,
            )
            .to::<Expression, Node>(arguments_node);

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
            let node = prefix_exp.child(0).unwrap();
            match node.kind() {
                "name" => {
                    PrefixExp::Var(Var::Name(node.utf8_text(code_bytes).unwrap().to_string()))
                }
                _ => PrefixExp::Var(Var::TableAccess(handle_table_var(node, code_bytes))),
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
