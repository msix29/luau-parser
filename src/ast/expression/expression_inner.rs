//! Implements various functions for _[inner expressions](ExpressionInner), mainly `From<>`
//! traits.

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{
        parse_block, Ast, ElseIfExpression, Expression, ExpressionInner, FunctionName,
        FunctionValue, List, ListItem, PrefixExp, SingleToken, TableField, TableFieldValue,
        TableKey, TableValue, TypeDefinition,
    },
    utils::get_location,
};

use crate::prelude::type_definition::functions::{
    build_function_parameters, build_function_returns,
};

use super::handle_prefix_exp::handle_prefix_exp;

pub(crate) fn build_table(node: Node, code_bytes: &[u8]) -> TableValue {
    let mut index = 0;
    let field_list = node.child_by_field_name("fieldList").unwrap();
    let separators = field_list
        .children_by_field_name("sep", &mut node.walk())
        .collect::<Vec<Node>>();

    TableValue {
        opening_brackets: SingleToken::from((
            node.child_by_field_name("opening_brackets").unwrap(),
            code_bytes,
        )),
        fields: Arc::new(
            field_list
                .children_by_field_name("field", &mut node.walk())
                .enumerate()
                .map(|(i, node)| {
                    let (key, key_location) = if let Some(key) = node.child_by_field_name("keyName")
                    {
                        (
                            TableKey::String(key.utf8_text(code_bytes).unwrap().to_string()),
                            Some(get_location(key)),
                        )
                    } else if let Some(key) = node.child_by_field_name("keyExp") {
                        (
                            TableKey::Expression {
                                open_square_brackets: SingleToken::from((
                                    key.prev_sibling().unwrap(),
                                    code_bytes,
                                )),
                                expression: Arc::new(Expression::from((key, code_bytes))),
                                close_square_brackets: SingleToken::from((
                                    key.next_sibling().unwrap(),
                                    code_bytes,
                                )),
                            },
                            Some(get_location(key)),
                        )
                    } else {
                        index += 1;
                        (TableKey::String(index.to_string()), None)
                    };
                    let value_node = node.child_by_field_name("value").unwrap();
                    let value = Expression::from((value_node, code_bytes));
                    TableField {
                        key: Arc::new(key),
                        key_location,
                        value_location: get_location(value_node),
                        location: get_location(node),
                        equal_or_colon: node
                            .child_by_field_name("equal")
                            .map(|node| SingleToken::from((node, code_bytes))),
                        r#type: None,
                        value: Some(Arc::new(TableFieldValue::Expression(value))),
                        separator: separators
                            .get(i)
                            .map(|node| SingleToken::from((*node, code_bytes))),
                    }
                })
                .collect::<Vec<TableField>>(),
        ),
        closing_brackets: SingleToken::from((
            node.child_by_field_name("opening_brackets").unwrap(),
            code_bytes,
        )),
        location: get_location(node),
    }
}

impl From<(&str, Node<'_>)> for ExpressionInner {
    fn from((value, node): (&str, Node<'_>)) -> Self {
        //TODO: Handle other cases.
        Self::String(crate::prelude::SimpleValue {
            location: get_location(node),
            value: value.to_string(),
        })
    }
}
impl ExpressionInner {
    /// Builds a list of _[inner expressions](ExpressionInner)_ from an iterator over nodes.
    ///
    /// # Note
    ///
    /// This function assumes the passed iter iterates over valid expression nodes.
    pub(crate) fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> List<ExpressionInner> {
        let nodes = nodes_iter.collect::<Vec<Node>>();
        if nodes.is_empty() {
            return List::default();
        }

        let last_index = nodes.len() - 1;

        List {
            items: nodes
                .iter()
                .enumerate()
                .step_by(2)
                .map(|(i, node)| {
                    if i == last_index {
                        ListItem::NonTrailing(ExpressionInner::from((*node, code_bytes)))
                    } else {
                        ListItem::Trailing {
                            item: ExpressionInner::from((*node, code_bytes)),
                            separator: SingleToken::from((nodes[i + 1], code_bytes)),
                        }
                    }
                })
                .collect(),
        }
    }
}

impl From<PrefixExp> for ExpressionInner {
    fn from(value: PrefixExp) -> Self {
        match value {
            PrefixExp::Var(var) => ExpressionInner::Var(var),
            PrefixExp::FunctionCall(function_call) => ExpressionInner::FunctionCall(function_call),
            PrefixExp::ExpressionWrap(expression_wrap) => {
                ExpressionInner::ExpressionWrap(expression_wrap)
            }
        }
    }
}

impl From<(Node<'_>, &[u8])> for ExpressionInner {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        match node.kind() {
            "nil" => ExpressionInner::from(("nil", node)),
            "boolean" => ExpressionInner::from((node.utf8_text(code_bytes).unwrap(), node)),
            "number" => ExpressionInner::from((node.utf8_text(code_bytes).unwrap(), node)),
            "string" => ExpressionInner::from((node.utf8_text(code_bytes).unwrap(), node)),
            "string_interp" => ExpressionInner::from((node.utf8_text(code_bytes).unwrap(), node)),
            "anon_fn" => {
                let mut ast_tokens = Vec::new();
                if let Some(body) = node.child_by_field_name("body") {
                    parse_block(body, &mut ast_tokens, code_bytes);
                }

                ExpressionInner::Function(FunctionValue {
                    local_keyword: None,
                    function_keyword: Some(SingleToken::from((
                        node.child_by_field_name("function").unwrap(),
                        code_bytes,
                    ))),
                    function_name: FunctionName::Anonymous,
                    parameters: Arc::new(build_function_parameters(node, code_bytes, false)),
                    returns: Arc::new(build_function_returns(node, code_bytes)),
                    body: Arc::new(Ast {
                        tokens: Arc::new(ast_tokens),
                        uri: None,
                    }),
                    end_keyword: Some(SingleToken::from((
                        node.child_by_field_name("end").unwrap(),
                        code_bytes,
                    ))),
                })
            }
            "var" | "functionCall" | "exp_wrap" => {
                ExpressionInner::from(handle_prefix_exp(node, code_bytes))
            }
            "table" => ExpressionInner::Table(build_table(node, code_bytes)),
            "unexp" => ExpressionInner::UnaryExpression {
                operator: SingleToken::from((node.child_by_field_name("op").unwrap(), code_bytes)),
                expression: Arc::new(Expression::from((
                    node.child_by_field_name("arg").unwrap(),
                    code_bytes,
                ))),
            },
            "binexp" => ExpressionInner::BinaryExpression {
                left: Arc::new(Expression::from((
                    node.child_by_field_name("arg0").unwrap(),
                    code_bytes,
                ))),
                operator: SingleToken::from((node.child_by_field_name("op").unwrap(), code_bytes)),
                right: Arc::new(Expression::from((
                    node.child_by_field_name("arg1").unwrap(),
                    code_bytes,
                ))),
            },
            "cast" => {
                let node = node.child_by_field_name("arg").unwrap();
                ExpressionInner::Cast {
                    expression: Arc::new(Expression::from((node, code_bytes))),
                    cast_to: Arc::new(TypeDefinition::from((
                        node.child_by_field_name("cast").unwrap(),
                        code_bytes,
                        false,
                    ))),
                    operator: SingleToken::from((
                        node.child_by_field_name("op").unwrap(),
                        code_bytes,
                    )),
                }
            }
            "ifexp" => ExpressionInner::IfExpression {
                if_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
                then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),

                else_if_expressions: Arc::new(
                    node.children_by_field_name("elseif", &mut node.walk())
                        .map(|node| ElseIfExpression {
                            else_if_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                            condition: Arc::new(Expression::from((
                                node.child(1).unwrap(),
                                code_bytes,
                            ))),
                            then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),
                            expression: Arc::new(Expression::from((
                                node.child(3).unwrap(),
                                code_bytes,
                            ))),
                        })
                        .collect::<Vec<ElseIfExpression>>(),
                ),

                else_token: SingleToken::from((
                    node.child_by_field_name("else").unwrap(),
                    code_bytes,
                )),
                else_expression: Arc::new(Expression::from((
                    node.child_by_field_name("elseExpression").unwrap(),
                    code_bytes,
                ))),
            },
            _ => todo!(
                "This should never be reached. But it did? Node: {}",
                node.to_sexp()
            ),
        }
    }
}
