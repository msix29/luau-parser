//! Implements various `From<>` traits for [`Expression`]s.

pub(crate) mod handle_prefix_exp;

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{
        parse_block, type_definition::helper_functions::build_generics, Ast, ElseIfExpression,
        Expression, HasRange, List, ListItem, Number, PrefixExp, Range, StringLiteral, Table,
        TableField, TableFieldValue, TableKey, Token, TypeDefinition,
    },
    utils::get_range_from_boundaries,
};

use crate::prelude::type_definition::helper_functions::{
    build_function_parameters, build_function_returns,
};

use handle_prefix_exp::handle_prefix_exp;

/// Build a table value from a node representing a table in an expression.
pub(crate) fn build_table(node: Node, code_bytes: &[u8]) -> Table {
    let mut index = 0;

    Table {
        opening_brackets: Token::from((
            node.child_by_field_name("opening_brackets").unwrap(),
            code_bytes,
        )),

        fields: node
            .child_by_field_name("fieldList")
            .map(|field_list| {
                List::from_iter(
                    field_list.children_by_field_name("field", &mut node.walk()),
                    field_list,
                    "sep",
                    code_bytes,
                    |_, node| {
                        let key = if let Some(key) = node.child_by_field_name("keyName") {
                            TableKey::String(StringLiteral::from((key, code_bytes)))
                        } else if let Some(key) = node.child_by_field_name("keyExp") {
                            TableKey::Expression {
                                open_square_brackets: Token::from((
                                    key.prev_sibling().unwrap(),
                                    code_bytes,
                                )),
                                expression: Arc::new(Expression::from((key, code_bytes))),
                                close_square_brackets: Token::from((
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
                                .map(|node| Token::from((node, code_bytes))),
                            value: Arc::new(TableFieldValue::Expression(value)),
                        }
                    },
                )
            })
            .unwrap_or_default(),

        closing_brackets: Token::from((
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
    ) -> List<Arc<Expression>> {
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
                        ListItem::NonTrailing(Arc::new(Expression::from((*node, code_bytes))))
                    } else {
                        ListItem::Trailing {
                            item: Arc::new(Expression::from((*node, code_bytes))),
                            separator: Token::from((nodes[i + 1], code_bytes)),
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
        if node.is_error() | node.is_missing() {
            return Self::ERROR;
        }

        match node.kind() {
            "nil" => Expression::Nil(Token::from((node, code_bytes))),
            "boolean" => Expression::Boolean(Token::from((node, code_bytes))),
            "number" => Expression::Number(Number::from((node, code_bytes))),
            "string" => Expression::String(StringLiteral::from((node, code_bytes))),
            "string_interp" => Expression::String(StringLiteral::from((node, code_bytes))),
            "anon_fn" => {
                Expression::Function {
                    function_keyword: Token::from((
                        node.child_by_field_name("function").unwrap(),
                        code_bytes,
                    )),
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
                    returns: build_function_returns(node, code_bytes).map(Arc::new),
                    colon: node
                        .child_by_field_name("colon")
                        .map(|colon| Token::from((colon, code_bytes))),
                    body: node
                        .child_by_field_name("body")
                        .map(|body| parse_block(&body, code_bytes, None))
                        .unwrap_or_default(),
                    //  Ast {
                    //     statements: Arc::new(ast_tokens),
                    //     uri: None,
                    // },
                    end_keyword: Token::from((
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
                operator: Token::from((node.child_by_field_name("op").unwrap(), code_bytes)),
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
                operator: Token::from((node.child_by_field_name("op").unwrap(), code_bytes)),
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
                operator: Token::from((node.child_by_field_name("op").unwrap(), code_bytes)),
            },
            "ifexp" => Expression::IfExpression {
                if_token: Token::from((node.child(0).unwrap(), code_bytes)),
                condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
                then_token: Token::from((node.child(2).unwrap(), code_bytes)),
                if_expression: Arc::new(Expression::from((node.child(3).unwrap(), code_bytes))),

                else_if_expressions: Arc::new(
                    node.children_by_field_name("elseif", &mut node.walk())
                        .map(|node| ElseIfExpression {
                            else_if_token: Token::from((node.child(0).unwrap(), code_bytes)),
                            condition: Arc::new(Expression::from((
                                node.child(1).unwrap(),
                                code_bytes,
                            ))),
                            then_token: Token::from((node.child(2).unwrap(), code_bytes)),
                            expression: Arc::new(Expression::from((
                                node.child(3).unwrap(),
                                code_bytes,
                            ))),
                        })
                        .collect::<Vec<ElseIfExpression>>(),
                ),

                else_token: Token::from((node.child_by_field_name("else").unwrap(), code_bytes)),
                else_expression: Arc::new(Expression::from((
                    node.child_by_field_name("elseExpression").unwrap(),
                    code_bytes,
                ))),
            },
            _ => unreachable!(
                "This should never be reached. But it did? Node: {}",
                node.to_sexp()
            ),
        }
    }
}

impl HasRange for Expression {
    fn get_range(&self) -> Range {
        match self {
            Expression::ERROR => Range::default(),
            Expression::Nil(value) => value.get_range(),
            Expression::Boolean(value) => value.get_range(),
            Expression::Number(value) => value.get_range(),
            Expression::String(value) => value.get_range(),
            Expression::Function {
                function_keyword,
                end_keyword,
                ..
            } => get_range_from_boundaries(function_keyword.get_range(), end_keyword.get_range()),
            Expression::FunctionCall(value) => value.get_range(),
            Expression::ExpressionWrap(value) => value.get_range(),
            Expression::Var(value) => value.get_range(),
            Expression::Table(value) => value.get_range(),
            Expression::UnaryExpression {
                operator,
                expression,
            } => get_range_from_boundaries(operator.get_range(), expression.get_range()),
            Expression::BinaryExpression {
                left,
                operator: _,
                right,
            } => get_range_from_boundaries(left.get_range(), right.get_range()),
            Expression::Cast {
                expression,
                operator: _,
                cast_to,
            } => get_range_from_boundaries(expression.get_range(), cast_to.get_range()),
            Expression::IfExpression {
                if_token,
                else_expression,
                ..
            } => get_range_from_boundaries(if_token.get_range(), else_expression.get_range()),
        }
    }
}

impl HasRange for ElseIfExpression {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.else_if_token.get_range(), self.expression.get_range())
    }
}
