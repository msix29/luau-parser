//! Implements various `From<>` traits for [`Expression`]s.

pub(crate) mod handle_prefix_exp;

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{
        parse_block, type_definition::functions::build_generics, Ast, ElseIfExpression, Expression,
        HasLocation, List, ListItem, Location, PrefixExp, SingleToken, Table, TableField,
        TableFieldValue, TableKey, TypeDefinition,
    },
    utils::get_location_from_boundaries,
};

use crate::prelude::type_definition::functions::{
    build_function_parameters, build_function_returns,
};

use handle_prefix_exp::handle_prefix_exp;

/// Build a table value from a node representing a table in an expression.
pub(crate) fn build_table(node: Node, code_bytes: &[u8]) -> Table {
    let mut index = 0;
    let field_list = node.child_by_field_name("fieldList").unwrap();

    Table {
        opening_brackets: SingleToken::from((
            node.child_by_field_name("opening_brackets").unwrap(),
            code_bytes,
        )),

        fields: List::from_iter(
            field_list.children_by_field_name("field", &mut node.walk()),
            field_list,
            "sep",
            code_bytes,
            |_, node| {
                let key = if let Some(key) = node.child_by_field_name("keyName") {
                    TableKey::String(SingleToken::from((key, code_bytes)))
                } else if let Some(key) = node.child_by_field_name("keyExp") {
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
                    }
                } else {
                    index += 1;
                    TableKey::UndefinedNumber(index)
                };
                let value_node = node.child_by_field_name("value").unwrap();
                let value = Expression::from((value_node, code_bytes));
                TableField {
                    key: Arc::new(key),
                    equal_or_colon: node
                        .child_by_field_name("equal")
                        .map(|node| SingleToken::from((node, code_bytes))),
                    value: Arc::new(TableFieldValue::Expression(value)),
                }
            },
        ),

        closing_brackets: SingleToken::from((
            node.child_by_field_name("closing_brackets").unwrap(),
            code_bytes,
        )),
    }
}

impl Expression {
    /// Builds a list of [`expressions`](Expression) from an iterator over nodes.
    ///
    /// # Note
    ///
    /// This function assumes the passed iter iterates over valid expression nodes.
    pub(crate) fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> List<Expression> {
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
                        ListItem::NonTrailing(Expression::from((*node, code_bytes)))
                    } else {
                        ListItem::Trailing {
                            item: Expression::from((*node, code_bytes)),
                            separator: SingleToken::from((nodes[i + 1], code_bytes)),
                        }
                    }
                })
                .collect(),
        }
    }

    /// Try getting th body of this expression, will only be `Some` if this expression
    /// is a function.
    pub fn try_get_body(&self) -> Option<&Ast> {
        match self {
            Expression::Function { body, .. } => Some(body),
            _ => None,
        }
    }
}

impl From<PrefixExp> for Expression {
    fn from(value: PrefixExp) -> Self {
        match value {
            PrefixExp::Var(var) => Expression::Var(var),
            PrefixExp::FunctionCall(function_call) => Expression::FunctionCall(function_call),
            PrefixExp::ExpressionWrap(expression_wrap) => {
                Expression::ExpressionWrap(expression_wrap)
            }
        }
    }
}

impl From<(Node<'_>, &[u8])> for Expression {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        match node.kind() {
            "nil" => Expression::Nil(SingleToken::from((node, code_bytes))),
            "boolean" => Expression::Boolean(SingleToken::from((node, code_bytes))),
            "number" => Expression::Number(SingleToken::from((node, code_bytes))),
            "string" => Expression::String(SingleToken::from((node, code_bytes))),
            "string_interp" => Expression::String(SingleToken::from((node, code_bytes))),
            "anon_fn" => {
                let mut ast_tokens = Vec::new();
                if let Some(body) = node.child_by_field_name("body") {
                    parse_block(body, &mut ast_tokens, code_bytes);
                }

                Expression::Function {
                    function_keyword: SingleToken::from((
                        node.child_by_field_name("function").unwrap(),
                        code_bytes,
                    )),
                    generics: build_generics(node, code_bytes),
                    opening_parenthesis: SingleToken::from((
                        node.child_by_field_name("opening_parenthesis").unwrap(),
                        code_bytes,
                    )),

                    parameters: build_function_parameters(node, code_bytes, false),
                    closing_parenthesis: SingleToken::from((
                        node.child_by_field_name("closing_parenthesis").unwrap(),
                        code_bytes,
                    )),
                    returns: Arc::new(build_function_returns(node, code_bytes)),
                    body: Ast {
                        tokens: Arc::new(ast_tokens),
                        uri: None,
                    },
                    end_keyword: SingleToken::from((
                        node.child_by_field_name("end").unwrap(),
                        code_bytes,
                    )),
                }
            }
            "var" | "functionCall" | "exp_wrap" => {
                Expression::from(handle_prefix_exp(node, code_bytes))
            }
            "table" => Expression::Table(build_table(node, code_bytes)),
            "unexp" => Expression::UnaryExpression {
                operator: SingleToken::from((node.child_by_field_name("op").unwrap(), code_bytes)),
                expression: Arc::new(Expression::from((
                    node.child_by_field_name("arg").unwrap(),
                    code_bytes,
                ))),
            },
            "binexp" => Expression::BinaryExpression {
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
            "cast" => Expression::Cast {
                expression: Arc::new(Expression::from((
                    node.child_by_field_name("arg").unwrap(),
                    code_bytes,
                ))),
                cast_to: Arc::new(TypeDefinition::from((
                    node.child_by_field_name("cast").unwrap(),
                    code_bytes,
                    false,
                ))),
                operator: SingleToken::from((node.child_by_field_name("op").unwrap(), code_bytes)),
            },
            "ifexp" => Expression::IfExpression {
                if_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
                then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),
                if_expression: Arc::new(Expression::from((node.child(3).unwrap(), code_bytes))),

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

impl HasLocation for Expression {
    fn get_location(&self) -> Location {
        match self {
            Expression::Nil(value) => value.get_location(),
            Expression::Boolean(value) => value.get_location(),
            Expression::Number(value) => value.get_location(),
            Expression::String(value) => value.get_location(),
            Expression::Function {
                function_keyword,
                end_keyword,
                ..
            } => get_location_from_boundaries(
                function_keyword.get_location(),
                end_keyword.get_location(),
            ),
            Expression::FunctionCall(value) => value.get_location(),
            Expression::ExpressionWrap(value) => value.get_location(),
            Expression::Var(value) => value.get_location(),
            Expression::Table(value) => value.get_location(),
            Expression::UnaryExpression {
                operator,
                expression,
            } => get_location_from_boundaries(operator.get_location(), expression.get_location()),
            Expression::BinaryExpression {
                left,
                operator: _,
                right,
            } => get_location_from_boundaries(left.get_location(), right.get_location()),
            Expression::Cast {
                expression,
                operator: _,
                cast_to,
            } => get_location_from_boundaries(expression.get_location(), cast_to.get_location()),
            Expression::IfExpression {
                if_token,
                else_expression,
                ..
            } => get_location_from_boundaries(
                if_token.get_location(),
                else_expression.get_location(),
            ),
        }
    }
}

impl HasLocation for ElseIfExpression {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.else_if_token.get_location(),
            self.expression.get_location(),
        )
    }
}
